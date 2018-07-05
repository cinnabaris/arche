use std::env::current_dir;
use std::fs;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use base64;
use log;
use mustache;
use rocket;
use toml;

use super::super::{cache, env, errors::Result, oauth, orm, queue, storage, utils};

pub fn nginx() -> Result<()> {
    let tpl = mustache::compile_path(Path::new("templates").join("nginx.conf"))?;
    let cur = current_dir()?;
    let cfg = super::parse_config()?;

    let file = Path::new("tmp").join("nginx.conf");
    log::info!("generate file {}", file.display());
    let mut fd = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o644)
        .open(file)?;
    tpl.render(
        &mut fd,
        &json!({
                "name": cfg.name,
                "port": cfg.http.port,
                "root": cur,
                "version":"v1",
            }),
    )?;

    Ok(())
}

pub fn config() -> Result<()> {
    let localhost = "localhost";

    let cfg = env::Config {
        name: String::from("www.change-me.com"),
        secret_key: base64::encode(&utils::random::bytes(32)),
        env: format!("{}", rocket::config::Environment::Development),

        http: env::Http {
            theme: String::from("bootstrap"),
            workers: 32,
            logging_level: format!("{}", rocket::config::LoggingLevel::Debug),
            port: 8080,
            limits: 1 << 16,
        },
        oauth: oauth::Config {
            line: Some(oauth::line::Config {
                channel_id: String::from("change-me"),
                channel_secret: String::from("change-me"),
                callback_url: String::from("http://localhost:3000/my/oauth/line"),
            }),
        },
        #[cfg(feature = "postgresql")]
        database: orm::Config {
            host: String::from(localhost),
            port: 5432,
            name: String::from(env::NAME),
            user: String::from("postgres"),
            password: String::from(""),
        },
        #[cfg(feature = "mysql")]
        database: orm::Config {
            host: String::from(localhost),
            port: 3306,
            name: String::from(env::NAME),
            user: String::from("root"),
            password: String::from(""),
        },
        cache: cache::Config {
            namespace: String::from("www.change-me.com"),
            redis: Some(cache::Redis {
                host: String::from(localhost),
                port: 6379,
                db: 6,
                password: None,
            }),
        },
        queue: queue::Config {
            name: String::from("tasks"),
            rabbitmq: Some(queue::RabbitMQ {
                host: String::from(localhost),
                port: 5672,
                virtual_: String::from(env::NAME),
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
            nfs: Some(storage::Nfs {
                end_point: String::from("/upload"),
                local_root: String::from("tmp/upload"),
            }),
            s3: Some(storage::S3 {
                region: String::from("us-west-2"),
                bucket: String::from("www.change-me.com"),
            }),
        },
    };
    let buf = toml::to_vec(&cfg)?;

    let file = super::config_file();
    log::info!("generate file {}", file.display());
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(file)?;
    file.write_all(&buf)?;

    Ok(())
}
