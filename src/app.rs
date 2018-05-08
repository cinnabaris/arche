use std::env::current_dir;
use std::io::{Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{fs, thread};

use base64;
use clap;
use diesel::Connection as DieselConnection;
use handlebars::Handlebars;
use log;
use rocket;
use rocket_contrib::Template;
use sys_info;
use toml;

use super::context::Context;
use super::env;
use super::result::Result;

pub struct App {
    // ctx: Context,
}

impl App {
    pub fn main() -> Result<()> {
        let generate_nginx =
            clap::SubCommand::with_name("generate:nginx").about("Generate nginx.conf");
        let generate_config =
            clap::SubCommand::with_name("generate:config").about("Generate config file");

        let db_migrate = clap::SubCommand::with_name("db:migrate")
            .about("Migrate the schema back to the latest version");
        let db_rollback = clap::SubCommand::with_name("db:rollback")
            .about("Rolls the schema back to the previous version");
        let db_seed = clap::SubCommand::with_name("db:seed").about("Loads the seed data");
        let db_dump = clap::SubCommand::with_name("db:dump").about("Dumps database as json files");
        let db_version = clap::SubCommand::with_name("db:version")
            .about("Retrieves the current schema version number");
        let db_restore = clap::SubCommand::with_name("db:restore")
            .about("Restores database from file")
            .arg(
                clap::Arg::with_name("name")
                    .short("n")
                    .help("Sets a custom config file")
                    .takes_value(true),
            );
        let routes = clap::SubCommand::with_name("routes")
            .about("Print out all defined routes in match order");

        let matches = clap::App::new(env::NAME)
            .version(env::VERSION)
            .author(env::AUTHORS)
            .about(env::DESCRIPTION)
            .before_help(env::BANNER)
            .after_help(env::HOMEPAGE)
            .subcommand(generate_nginx)
            .subcommand(generate_config)
            .subcommand(db_rollback)
            .subcommand(db_migrate)
            .subcommand(db_seed)
            .subcommand(db_dump)
            .subcommand(db_version)
            .subcommand(db_restore)
            .subcommand(routes)
            .get_matches();

        if let Some(_) = matches.subcommand_matches("generate:config") {
            return Self::generate_config();
        }
        if let Some(_) = matches.subcommand_matches("routes") {
            return Self::routes();
        }
        let app = Self::new()?;
        if let Some(_) = matches.subcommand_matches("generate:nginx") {
            return app.generate_nginx();
        }
        if let Some(_) = matches.subcommand_matches("db:migrate") {
            return app.db_migrate();
        }
        if let Some(_) = matches.subcommand_matches("db:rollback") {
            return app.db_rollback();
        }
        if let Some(_) = matches.subcommand_matches("db:seed") {
            return app.db_seed();
        }
        if let Some(_) = matches.subcommand_matches("db:dump") {
            return app.db_dump();
        }
        if let Some(_) = matches.subcommand_matches("db:version") {
            return app.db_version();
        }
        if let Some(matches) = matches.subcommand_matches("db:restore") {
            if let Some(name) = matches.value_of("name") {
                return app.db_restore(&s!(name));
            }
        }

        return app.server();
    }
}

impl App {
    const CONFIG_FILE: &'static str = "config.toml";

    fn new() -> Result<Self> {
        // TODO
        Ok(Self {})
    }
    fn generate_config() -> Result<()> {
        use super::security::{Encryptor, Sodium};
        use sodiumoxide::randombytes;

        let localhost = "localhost";

        let cfg = env::Config {
            name: s!("www.change-me.com"),
            secret_key: base64::encode(&randombytes::randombytes(32)),
            env: rocket::config::Environment::Development.to_string(),
            languages: vec![s!("en-US"), s!("zh-Hans"), s!("zh-Hant")],
            workers: 32,
            http: env::Http {
                theme: s!("bootstrap"),
                limits: 1 << 25,
                port: 8080,
                origins: vec![s!("http://localhost:3000")],
            },
            database: env::Database {
                postgresql: Some(env::PostgreSql {
                    host: s!(localhost),
                    port: 5432,
                    name: s!(env::NAME),
                    user: s!("postgres"),
                    password: s!(""),
                }),
                mysql: Some(env::MySql {
                    host: s!(localhost),
                    port: 3306,
                    name: s!(env::NAME),
                    user: s!("postgres"),
                    password: s!(""),
                }),
            },
            cache: env::Cache {
                namespace: s!("www.change-me.com"),
                redis: Some(env::Redis {
                    host: s!(localhost),
                    port: 6379,
                    db: 6,
                    password: None,
                }),
            },
            queue: env::Queue {
                name: s!("tasks"),
                rabbitmq: Some(env::RabbitMQ {
                    host: s!(localhost),
                    port: 5672,
                    _virtual: s!(env::NAME),
                    user: s!("guest"),
                    password: s!("guest"),
                }),
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

        log::info!("generate file {}", Self::CONFIG_FILE);
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(Self::CONFIG_FILE)?;
        file.write_all(&buf)?;

        Ok(())
    }
    fn routes() -> Result<()> {
        // TODO
        Ok(())
    }
    fn generate_nginx(&self) -> Result<()> {
        // TODO
        Ok(())
    }
    fn db_migrate(&self) -> Result<()> {
        // TODO
        Ok(())
    }
    fn db_rollback(&self) -> Result<()> {
        // TODO
        Ok(())
    }
    fn db_seed(&self) -> Result<()> {
        // TODO
        Ok(())
    }
    fn db_dump(&self) -> Result<()> {
        // TODO
        Ok(())
    }
    fn db_version(&self) -> Result<()> {
        // TODO
        Ok(())
    }
    fn db_restore(&self, _name: &String) -> Result<()> {
        // TODO
        Ok(())
    }
    fn server(&self) -> Result<()> {
        // TODO
        Ok(())
    }
}
