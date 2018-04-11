use clap;
use sodiumoxide;

use super::result::Result;
use super::{app, env};

pub fn run() -> Result<()> {
    sodiumoxide::init();

    let generate_nginx = clap::SubCommand::with_name("generate:nginx").about("Generate nginx.conf");
    let generate_config =
        clap::SubCommand::with_name("generate:config").about("Generate config file(toml)");
    let i18n_sync = clap::SubCommand::with_name("i18n:sync")
        .about("Sync locale items from filesystem")
        .arg(
            clap::Arg::with_name("dir")
                .short("d")
                .help("Locales dir")
                .required(true)
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
        .subcommand(i18n_sync)
        .get_matches();

    if let Some(_) = matches.subcommand_matches("generate:nginx") {
        return app::generate_nginx();
    }
    if let Some(_) = matches.subcommand_matches("generate:config") {
        return app::generate_config();
    }
    if let Some(matches) = matches.subcommand_matches("i18n:sync") {
        if let Some(dir) = matches.value_of("dir") {
            return app::i18n_sync(String::from(dir));
        }
    }

    return app::server();
}
