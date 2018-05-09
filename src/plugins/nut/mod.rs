pub mod consumers;
pub mod controllers;
pub mod models;
pub mod schema;
pub mod seed;

use rocket::Route;
use serde_json::Value;

use super::super::result::Result;

pub fn application_layout(lang: &String) -> Result<Value> {
    Ok(json!({ "locale": lang }))
}

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    vec![(
        "/",
        routes!(
            controllers::index,
            controllers::sitemap,
            controllers::robots,
            controllers::rss
        ),
    )]
}
