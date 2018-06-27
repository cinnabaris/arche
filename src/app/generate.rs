use std::env::current_dir;
use std::fs;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use base64;
use mustache;
use toml;

use super::super::{cache, env, errors::Result, oauth, orm, queue, storage, utils};

pub fn nginx() -> Result<()> {
    let tpl = mustache::compile_path(Path::new("templates").join("nginx.conf"))?;
    let cur = current_dir()?;
    let cfg = super::parse_config()?;

    let file = Path::new("tmp").join("nginx.conf");
    info!("generate file {}", file.display());
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
        env: env::Environment::Development,

        http: env::Http {
            theme: String::from("bootstrap"),
            port: 8080,
        },
        oauth: oauth::Config {
            line: Some(oauth::line::Config {
                channel_id: String::from("change-me"),
                channel_secret: String::from("change-me"),
                callback_url: String::from("http://localhost:3000/my/oauth/line"),
            }),
        },
        #[cfg(feature = "db-pg")]
        database: orm::Config {
            host: String::from(localhost),
            port: 5432,
            name: String::from(env::NAME),
            user: String::from("postgres"),
            password: String::from(""),
        },
        #[cfg(feature = "db-my")]
        database: orm::Config {
            host: String::from(localhost),
            port: 3306,
            name: String::from(env::NAME),
            user: String::from("postgres"),
            password: String::from(""),
        },
        #[cfg(feature = "ch-redis")]
        cache: cache::Config {
            namespace: String::from("www.change-me.com"),
            host: String::from(localhost),
            port: 6379,
            db: 6,
            password: None,
        },
        #[cfg(feature = "mq-rabbit")]
        queue: queue::Config {
            host: String::from(localhost),
            port: 5672,
            virtual_: String::from(env::NAME),
            name: String::from("tasks"),
            user: String::from("guest"),
            password: String::from("guest"),
        },
        #[cfg(any(feature = "ch-aws", feature = "st-aws", feature = "mq-aws"))]
        aws: env::Aws {
            access_key_id: String::from("change-me"),
            secret_access_key: String::from("change-me"),
        },
        elasticsearch: env::ElasticSearch {
            hosts: vec![String::from("http://localhost:9200")],
        },
        #[cfg(feature = "st-fs")]
        storage: storage::Config {
            end_point: String::from("/upload"),
            local_root: String::from("tmp/upload"),
        },
        #[cfg(feature = "st-aws")]
        storage: storage::Config {
            region: String::from("us-west-2"),
            bucket: String::from("www.change-me.com"),
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
