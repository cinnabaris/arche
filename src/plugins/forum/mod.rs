pub mod controllers;

use chrono::Utc;
use rocket::Route;
use sitemap::structs::ChangeFreq;

use super::super::{RssItem, SitemapItem};

pub fn routes() -> (&'static str, Vec<Route>) {
    ("/forum", routes![controllers::home])
}

pub fn sitemap() -> Vec<SitemapItem> {
    vec![(
        "/forum".to_string(),
        0.1,
        ChangeFreq::Daily,
        Utc::now().naive_utc(),
    )]
}

pub fn rss(_lang: &String) -> Vec<RssItem> {
    vec![(
        "/forum/posts/1".to_string(),
        "post title 1".to_string(),
        "post body 1".to_string(),
        Utc::now().naive_utc(),
    )]
}
