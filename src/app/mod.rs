pub mod cache;
pub mod db;
pub mod generate;
pub mod http;
pub mod worker;

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use clap;
use sodiumoxide;
use toml;

use super::{context::Context, env, errors::Result};

pub fn main() -> Result<()> {
    sodiumoxide::init();

    let generate_nginx = clap::SubCommand::with_name("generate:nginx").about("Generate nginx.conf");
    let generate_config =
        clap::SubCommand::with_name("generate:config").about("Generate config file");

    let db_versions = clap::SubCommand::with_name("db:versions")
        .about("Retrieves the current schema version number");

    let routes =
        clap::SubCommand::with_name("routes").about("Print out all defined routes in match order");

    let cache_clear = clap::SubCommand::with_name("cache:clear").about("Clear all cache items");
    let cache_list = clap::SubCommand::with_name("cache:list").about("List all cache items");

    let matches = clap::App::new(env::NAME)
        .version(env::VERSION)
        .author(env::AUTHORS)
        .about(env::DESCRIPTION)
        .before_help(env::BANNER)
        .after_help(env::HOMEPAGE)
        .subcommand(generate_nginx)
        .subcommand(generate_config)
        .subcommand(db_versions)
        .subcommand(cache_list)
        .subcommand(cache_clear)
        .subcommand(routes)
        .get_matches();

    if let Some(_) = matches.subcommand_matches("generate:config") {
        return generate::config();
    }
    if let Some(_) = matches.subcommand_matches("generate:nginx") {
        return generate::nginx();
    }
    if let Some(_) = matches.subcommand_matches("routes") {
        return http::routes();
    }
    if let Some(_) = matches.subcommand_matches("db:versions") {
        return db::versions();
    }
    if let Some(_) = matches.subcommand_matches("cache:clear") {
        return cache::clear();
    }
    if let Some(_) = matches.subcommand_matches("cache:list") {
        return cache::list();
    }

    // main entry;
    let cfg = parse_config()?;
    let ctx = Context::new(&cfg)?;
    let ctx = Arc::new(ctx);

    let http = cfg.http.clone();
    let que = cfg.queue.clone();
    let wrk = Arc::clone(&ctx);
    thread::spawn(move || {
        match worker::start(&que, Arc::clone(&wrk)) {
            Ok(_) => warn!("worker exit."),
            Err(e) => error!("failed in worker: {:?}", e),
        }
        thread::sleep(Duration::from_secs(10));
    });

    http::server(&http, Arc::clone(&ctx));
    Ok(())
}

fn config_file() -> PathBuf {
    return Path::new("config.toml").to_path_buf();
}

fn parse_config() -> Result<env::Config> {
    let file = config_file();
    info!("load config from file {}", file.display());
    let mut file = fs::File::open(file)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let cfg: env::Config = toml::from_slice(&buf)?;

    Ok(cfg)
}
