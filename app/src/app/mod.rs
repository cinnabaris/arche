pub mod cache;
pub mod db;
pub mod generate;
pub mod http;

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use clap;
use nut::{env, errors::Result};
use sodiumoxide;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

pub fn main() -> Result<()> {
    if cfg!(feature = "sodium") {
        sodiumoxide::init();
    }
    let generate_nginx = clap::SubCommand::with_name("generate:nginx").about("Generate nginx.conf");
    let generate_config =
        clap::SubCommand::with_name("generate:config").about("Generate config file");

    let db_versions = clap::SubCommand::with_name("db:versions")
        .about("Retrieves the current schema version number");

    let routes =
        clap::SubCommand::with_name("routes").about("Print out all defined routes in match order");

    let cache_clear = clap::SubCommand::with_name("cache:clear").about("Clear all cache items");
    let cache_list = clap::SubCommand::with_name("cache:list").about("List all cache items");

    let matches = clap::App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .before_help(env::BANNER)
        .after_help(HOMEPAGE)
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

    http::server()
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
