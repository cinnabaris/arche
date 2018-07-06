#![feature(plugin, use_extern_macros, custom_derive, extern_prelude)]
#![plugin(rocket_codegen)]
#![recursion_limit = "1024"]
extern crate rocket;
#[macro_use]
extern crate error_chain;
#[macro_use(log)]
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
#[macro_use]
extern crate lazy_static;

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
extern crate juniper_rocket;
extern crate language_tags;
extern crate lettre;
extern crate lettre_email;
extern crate log4rs;
extern crate maxminddb;
extern crate md5;
extern crate mime;
extern crate mustache;
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;
extern crate regex;
extern crate robots_txt;
extern crate rocket_contrib;
extern crate rocket_cors;
extern crate rss;
extern crate rusoto_core;
extern crate rusoto_elasticache;
extern crate rusoto_s3;
extern crate rusoto_sns;
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

use chrono::NaiveDateTime;
use sitemap::structs::ChangeFreq;

pub type SitemapItem = (String, f32, ChangeFreq, NaiveDateTime);
pub type RssItem = (String, String, String, NaiveDateTime);
