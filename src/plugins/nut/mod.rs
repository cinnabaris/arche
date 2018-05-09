pub mod consumers;
pub mod controllers;
pub mod models;
pub mod schema;
pub mod seed;

use std::collections::HashMap;

use chrono::{NaiveDateTime, Utc};
use rocket::Route;
use serde_json::Value;
use sitemap::structs::ChangeFreq;

use super::super::result::Result;

pub fn sitemap() -> Vec<(String, f32, ChangeFreq, NaiveDateTime)> {
    vec![(s!("/"), 0.1, ChangeFreq::Daily, Utc::now().naive_utc())]
}

pub fn application_layout(lang: &String) -> Result<HashMap<String, Value>> {
    let mut out = HashMap::new();
    out.insert(s!("locale"), json!(lang));
    Ok(out)
}

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    vec![("/", routes!(controllers::index,))]
}
