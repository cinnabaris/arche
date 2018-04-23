use clap;
use sodiumoxide;

use super::result::Result;
use super::{app, env};

pub fn run() -> Result<()> {
    sodiumoxide::init();

    let generate_nginx = clap::SubCommand::with_name("generate:nginx").about("Generate nginx.conf");
    let generate_config =
        clap::SubCommand::with_name("generate:config").about("Generate config file");
    let db_seed = clap::SubCommand::with_name("db:seed").about("Loads the seed data");
    let db_dump = clap::SubCommand::with_name("db:dump").about("Dumps database as json files");
    let db_restore = clap::SubCommand::with_name("db:restore")
        .about("Restores database from file")
        .arg(
            clap::Arg::with_name("name")
                .short("n")
                .help("Sets a custom config file")
                .takes_value(true),
        );

    let matches = clap::App::new(env::NAME)
        .version(env::VERSION)
        .author(env::AUTHORS)
        .about(env::DESCRIPTION)
        .before_help(env::BANNER)
        .after_help(env::HOMEPAGE)
        .subcommand(generate_nginx)
        .subcommand(generate_config)
        .subcommand(db_seed)
        .subcommand(db_dump)
        .subcommand(db_restore)
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
    if let Some(_) = matches.subcommand_matches("db:dump") {
        return app::db_dump();
    }
    if let Some(matches) = matches.subcommand_matches("db:restore") {
        if let Some(name) = matches.value_of("name") {
            return app::db_restore(&s!(name));
        }
    }

    return app::server();
}
