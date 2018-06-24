#![feature(
    plugin, use_extern_macros, custom_derive, custom_attribute, proc_macro_path_invoc,
    extern_prelude
)]
#![recursion_limit = "128"] // https://github.com/diesel-rs/diesel/issues/1127
#![plugin(rocket_codegen)]
extern crate rocket;

#[macro_use(log)]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate juniper;

#[cfg(feature = "mq-rabbit")]
extern crate amqp;
extern crate base64;
extern crate byteorder;
extern crate chrono;
extern crate clap;
extern crate csv;
extern crate epub;
extern crate eui48;
extern crate flate2;
extern crate frank_jwt;
extern crate geo;
extern crate handlebars;
extern crate hex;
extern crate hyper;
extern crate ini;
extern crate juniper_rocket;
extern crate language_tags;
extern crate lettre;
extern crate lettre_email;
extern crate log4rs;
extern crate maxminddb;
extern crate md5;
extern crate mime;
extern crate r2d2;
#[cfg(feature = "ch-redis")]
extern crate r2d2_redis;
#[cfg(feature = "ch-redis")]
extern crate redis;
extern crate regex;
extern crate robots_txt;
extern crate rocket_contrib;
extern crate rocket_cors;
extern crate rss;
#[cfg(any(feature = "mq-aws", feature = "st-aws"))]
extern crate rusoto_core;
#[cfg(feature = "st-aws")]
extern crate rusoto_s3;
#[cfg(feature = "mq-aws")]
extern crate rusoto_sns;
#[cfg(feature = "mq-aws")]
extern crate rusoto_sqs;
extern crate serde;
extern crate serde_xml_rs;
extern crate sha2;
extern crate sitemap;
#[cfg(feature = "sodium")]
extern crate sodiumoxide;
extern crate stardict;
extern crate sys_info;
extern crate toml;
extern crate url;
extern crate uuid;
extern crate validator;

#[macro_use]
pub mod macros;

pub mod app;
pub mod cache;
pub mod context;
pub mod dao;
pub mod env;
pub mod graphql;
pub mod i18n;
pub mod jwt;
pub mod pagination;
pub mod plugins;
pub mod queue;
pub mod result;
pub mod rfc;
pub mod router;
pub mod security;
pub mod settings;
pub mod themes;
