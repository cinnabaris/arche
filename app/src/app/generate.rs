use std::env::current_dir;
use std::fs;
use std::io::{Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use base64;
use handlebars::Handlebars;
use nut::{cache, env, errors::Result, orm, queue, security, storage};
use rocket::config::Environment;
use toml;

pub fn nginx() -> Result<()> {
    let cur = current_dir()?;
    let mut fd = fs::OpenOptions::new()
        .read(true)
        .open(Path::new("templates").join("nginx.conf.hbs"))?;
    let mut buf = String::new();
    fd.read_to_string(&mut buf)?;

    let cfg = super::parse_config()?;

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
    info!("generate file {}", file.display());
    let mut tpl = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o644)
        .open(file)?;
    tpl.write_all(body.as_bytes())?;

    Ok(())
}

pub fn config() -> Result<()> {
    let localhost = "localhost";

    let cfg = env::Config {
        name: String::from("www.change-me.com"),
        secret_key: base64::encode(&security::random_bytes(32)),
        env: Environment::Development.to_string(),
        languages: vec![
            String::from("en-US"),
            String::from("zh-Hans"),
            String::from("zh-Hant"),
        ],
        workers: 32,
        http: env::Http {
            theme: String::from("bootstrap"),
            limits: 1 << 25,
            port: 8080,
            origins: vec![
                String::from("http://localhost:3000"),
                String::from("http://localhost:8080"),
            ],
        },
        database: orm::Config {
            #[cfg(feature = "postgresql")]
            postgresql: orm::postgresql::Config {
                host: String::from(localhost),
                port: 5432,
                name: String::from(super::NAME),
                user: String::from("postgres"),
                password: String::from(""),
            },
            #[cfg(feature = "mysql")]
            mysql: orm::mysql::Config {
                host: String::from(localhost),
                port: 3306,
                name: String::from(super::NAME),
                user: String::from("postgres"),
                password: String::from(""),
            },
        },
        cache: cache::Config {
            namespace: String::from("www.change-me.com"),
            redis: Some(cache::redis::Config {
                host: String::from(localhost),
                port: 6379,
                db: 6,
                password: None,
            }),
        },
        queue: queue::Config {
            name: String::from("tasks"),
            rabbitmq: Some(queue::rabbitmq::Config {
                host: String::from(localhost),
                port: 5672,
                virtual_: String::from(super::NAME),
                user: String::from("guest"),
                password: String::from("guest"),
            }),
        },
        aws: env::Aws {
            access_key_id: String::from("change-me"),
            secret_access_key: String::from("change-me"),
        },
        elasticsearch: env::ElasticSearch {
            hosts: vec![String::from("http://localhost:9200")],
        },
        storage: storage::Config {
            nfs: Some(storage::nfs::Config {
                end_point: String::from("/upload"),
                local_root: String::from("tmp/upload"),
            }),
            s3: Some(storage::s3::Config {
                region: String::from("us-west-2"),
                bucket: String::from("www.change-me.com"),
            }),
        },
    };
    let buf = toml::to_vec(&cfg)?;

    let file = super::config_file();
    info!("generate file {}", file.display());
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(file)?;
    file.write_all(&buf)?;

    Ok(())
}
