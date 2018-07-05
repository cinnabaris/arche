pub mod consumers;
pub mod controllers;
pub mod dao;
pub mod graphql;

use chrono::Utc;
use rocket::Route;
use sitemap::structs::ChangeFreq;

use super::super::{RssItem, SitemapItem};

pub fn routes() -> (&'static str, Vec<Route>) {
    ("/", routes![controllers::home])
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
