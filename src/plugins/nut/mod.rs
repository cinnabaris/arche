pub mod consumers;
pub mod controllers;
pub mod dao;
pub mod graphql;
pub mod models;

use chrono::Utc;
use rocket::Route;
use sitemap::structs::ChangeFreq;

use super::super::{RssItem, SitemapItem};

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    vec![("/", routes![controllers::home])]
}

pub fn sitemap() -> Vec<SitemapItem> {
    vec![(
        String::from("/"),
        0.1,
        ChangeFreq::Daily,
        Utc::now().naive_utc(),
    )]
}

pub fn rss(_lang: &String) -> Vec<RssItem> {
    vec![]
}
