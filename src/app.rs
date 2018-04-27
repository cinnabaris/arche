use std::env::current_dir;
use std::io::{Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{fs, thread};

use base64;
use diesel::Connection as DieselConnection;
use frank_jwt::Algorithm;
use handlebars::Handlebars;
use log;
use rocket;
use rocket_contrib::Template;
use sys_info;
use toml;

use super::orm::{self, Connection as Db};
use super::queue::{self, Provider as QueueProvider};
use super::result::{Error, Result};
use super::{cache, env, graphql, i18n, jwt, nut, router, security, spree};

pub fn server() -> Result<()> {
    // worker
    thread::spawn(|| loop {
        let worker = || -> Result<()> {
            let name = sys_info::hostname()?;
            log::info!("Starting worker thread {}", name);
            let etc = parse_config()?;
            queue::new(etc.queue.url.clone(), etc.queue.name.clone()).consume(
                queue::Consumer::new(
                    name,
                    etc.env()?,
                    orm::new(etc.database.clone())?,
                    security::Encryptor::new(etc.secret_key()?.as_slice())?,
                ),
            )?;
            Ok(())
        };
        match worker() {
            Ok(_) => log::info!("exiting worker"),
            Err(e) => log::error!("{:?}", e),
        }
        thread::sleep(Duration::from_secs(10));
    });
    // http
    let etc = parse_config()?;
    let cfg = rocket::config::Config::build(etc.env()?)
        .address("localhost")
        .workers(etc.workers)
        .secret_key(etc.secret_key.clone())
        .port(etc.http.port)
        .limits(etc.http.limits())
        .extra(
            "template_dir",
            match Path::new("themes")
                .join(&etc.http.theme)
                .join("views")
                .to_str()
            {
                Some(v) => v,
                None => "views",
            },
        )
        .finalize()?;
    let dbp = orm::new(etc.database.clone())?;
    let mut app = rocket::custom(cfg, false)
        .manage(dbp.clone())
        .manage(graphql::schema::Schema::new(
            graphql::query::Query {},
            graphql::mutation::Mutation {},
        ))
        .manage(cache::new(&etc.cache.url, etc.cache.namespace.clone())?)
        .manage(security::Encryptor::new(etc.secret_key()?.as_slice())?)
        .manage(queue::new(etc.queue.url.clone(), etc.queue.name.clone()))
        .manage(jwt::Jwt::new(
            etc.secret_key()?.as_slice(),
            Algorithm::HS512,
        ))
        .manage(etc.clone());

    if !etc.is_prod() {
        app = app.mount("/", routes!(router::get_assets))
    }

    for (pt, rt) in router::routes() {
        app = app.mount(pt, rt);
    }

    app.attach(Template::fairing())
        .attach(etc.http.cors())
        .catch(router::catchers())
        .launch();

    Ok(())
}

pub fn generate_config() -> Result<()> {
    let cfg = env::Config {
        name: s!("www.change-me.com"),
        secret_key: base64::encode(&security::random_bytes(32)),
        env: rocket::config::Environment::Development.to_string(),
        languages: vec![s!("en-US"), s!("zh-Hans"), s!("zh-Hant")],
        workers: 32,
        http: env::Http {
            theme: s!("bootstrap"),
            limits: 1 << 25,
            port: 8080,
            origins: vec![s!("http://localhost:3000")],
        },
        #[cfg(feature = "postgresql")]
        database: s!("postgres://postgres:@localhost:5432/arche"),
        #[cfg(feature = "mysql")]
        database: s!("mysql://root:@localhost:3306/arche"),
        cache: env::Cache {
            namespace: s!("www.change-me.com"),
            url: s!("redis://:@localhost:6379/6"),
        },
        queue: env::Queue {
            url: s!("rabbitmq://guest:guest@localhost:5672/arche"),
            name: s!("tasks"),
        },
        aws: Some(env::Aws {
            access_key_id: s!("change-me"),
            secret_access_key: s!("change-me"),
        }),
        elasticsearch: env::ElasticSearch {
            hosts: vec![s!("http://localhost:9200")],
        },
        storage: env::Storage {
            local: Some(env::Local {
                end_point: s!("/upload"),
                local_root: s!("tmp/upload"),
            }),
            s3: Some(env::S3 {
                region: s!("us-west-2"),
                bucket: s!("www.change-me.com"),
            }),
        },
    };
    let buf = toml::to_vec(&cfg)?;

    let name = config_file();
    log::info!("generate file {}", name);
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(name)?;
    file.write_all(&buf)?;
    return Ok(());
}

pub fn generate_nginx() -> Result<()> {
    let cfg = parse_config()?;
    let cur = current_dir()?;
    let mut fd = fs::OpenOptions::new()
        .read(true)
        .open(Path::new("templates").join("nginx.conf.hbs"))?;
    let mut buf = String::new();
    fd.read_to_string(&mut buf)?;

    let body = Handlebars::new().render_template(
        &buf,
        &json!({
                "name": cfg.name,
                "port": cfg.http.port,
                "root": cur,
                "version":"v1",
            }),
    )?;

    let root = Path::new("tmp");
    fs::create_dir_all(&root)?;
    let file = root.join("nginx.conf");
    log::info!("generate file {}", file.display());
    let mut tpl = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o644)
        .open(file)?;
    tpl.write_all(body.as_bytes())?;

    return Ok(());
}

pub fn db_seed() -> Result<()> {
    let etc = parse_config()?;
    let pool = orm::new(etc.database)?;
    let db = orm::Connection(pool.get()?);
    let root = Path::new("db").join("seed");

    db.transaction::<_, Error, _>(|| {
        load_locales(&db, &root)?;
        spree::seed::load(&db, &root)?;
        nut::seed::administrator(&db)?;
        Ok(())
    })
}

pub fn db_dump() -> Result<()> {
    // TODO
    Ok(())
}

pub fn db_restore(_name: &String) -> Result<()> {
    // TODO
    Ok(())
}

//-----------------------------------------------------------------------------

fn load_locales(db: &Db, root: &PathBuf) -> Result<()> {
    let dir = root.join("locales");
    log::info!("load locales from {:?}...", &dir);
    let (total, inserted) = i18n::Locale::sync(db, dir)?;
    log::info!("total {}, inserted {}", total, inserted);
    Ok(())
}

fn parse_config() -> Result<env::Config> {
    let name = config_file();
    log::info!("load config from file {}", name);
    let mut file = fs::File::open(name)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let cfg: env::Config = toml::from_slice(&buf)?;
    return Ok(cfg);
}

fn config_file() -> &'static str {
    return "config.toml";
}
