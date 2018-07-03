#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
#[macro_use]
pub extern crate log;
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
// #[macro_use]
// extern crate lazy_static;

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
extern crate futures;
extern crate geo;
extern crate hex;
extern crate http;
extern crate hyper;
extern crate ini;
extern crate language_tags;
extern crate lettre;
extern crate lettre_email;
extern crate log4rs;
extern crate maxminddb;
extern crate md5;
extern crate mime;
extern crate mustache;
extern crate r2d2;
#[cfg(feature = "ch-redis")]
extern crate r2d2_redis;
#[cfg(feature = "ch-redis")]
extern crate redis;
extern crate regex;
extern crate robots_txt;
extern crate rss;
#[cfg(any(feature = "ch-aws", feature = "st-aws", feature = "mq-aws"))]
extern crate rusoto_core;
#[cfg(feature = "ch-aws")]
extern crate rusoto_elasticache;
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
extern crate sodiumoxide;
extern crate stardict;
extern crate toml;
extern crate url;
extern crate uuid;
extern crate validator;

#[macro_use]
pub mod macros;

pub mod app;
pub mod cache;
pub mod context;
pub mod env;
pub mod errors;
pub mod graphql;
pub mod i18n;
pub mod jwt;
pub mod oauth;
pub mod orm;
pub mod plugins;
pub mod queue;
pub mod request;
pub mod rfc;
pub mod router;
pub mod settings;
pub mod storage;
pub mod sys;
pub mod utils;
