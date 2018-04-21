use std::env::current_dir;
use std::io::{Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::{fs, thread};

use base64;
use frank_jwt::Algorithm;
use handlebars::Handlebars;
use log;
use rocket;
use rocket_contrib::Template;
use sys_info;
use toml;

use super::queue::{self, Provider as QueueProvider};
use super::result::Result;
use super::{cache, env, i18n, jwt, orm, router, security};

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
                    orm::pool(&etc.database)?,
                    security::Encryptor::new(etc.secret_key()?.as_slice())?,
                ),
            )?;
            Ok(())
        };
        match worker() {
            Ok(_) => log::info!("exiting worker"),
            Err(e) => log::error!("{:?}", e),
        }
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
    let mut app = rocket::custom(cfg, false)
        .manage(orm::pool(&etc.database)?)
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

pub fn i18n_sync(dir: String) -> Result<()> {
    let etc = parse_config()?;
    let pool = orm::pool(&etc.database)?;
    let db = orm::Connection(pool.get()?);
    let (total, inserted) = i18n::Locale::sync(&db, Path::new(&dir).to_path_buf())?;
    log::info!("total {}, inserted {}", total, inserted);
    return Ok(());
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

//-----------------------------------------------------------------------------

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
