#![feature(extern_prelude, plugin, attr_literals, custom_attribute)]
#![plugin(rocket_codegen)]
extern crate rocket;

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate juniper;

extern crate amqp;
extern crate base64;
extern crate chrono;
extern crate clap;
extern crate diesel;
extern crate handlebars;
extern crate juniper_rocket;
extern crate nut;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_xml_rs;
extern crate sodiumoxide;
extern crate survey;
extern crate toml;

pub mod app;
pub mod graphql;
pub mod router;
pub mod worker;
