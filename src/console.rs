use clap;
use sodiumoxide;

use super::result::Result;
use super::{app, env};

pub fn run() -> Result<()> {
    sodiumoxide::init();

    let generate_nginx = clap::SubCommand::with_name("generate:nginx").about("Generate nginx.conf");
    let generate_config =
        clap::SubCommand::with_name("generate:config").about("Generate config file(toml)");
    let db_seed = clap::SubCommand::with_name("db:seed").about("Loads the seed data(db/seed)");

    let matches = clap::App::new(env::NAME)
        .version(env::VERSION)
        .author(env::AUTHORS)
        .about(env::DESCRIPTION)
        .before_help(env::BANNER)
        .after_help(env::HOMEPAGE)
        .subcommand(generate_nginx)
        .subcommand(generate_config)
        .subcommand(db_seed)
        .get_matches();

    if let Some(_) = matches.subcommand_matches("generate:nginx") {
        return app::generate_nginx();
    }
    if let Some(_) = matches.subcommand_matches("generate:config") {
        return app::generate_config();
    }
    if let Some(_) = matches.subcommand_matches("db:seed") {
        return app::db_seed();
    }

    return app::server();
}
