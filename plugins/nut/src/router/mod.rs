pub mod errors;
pub mod home;

use chrono::Utc;
use rocket::{Catcher, Route};
use sitemap::structs::ChangeFreq;

use super::{RssItem, SitemapItem};

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    vec![(
        "/",
        routes!(home::robots_txt, home::assets, home::global, home::third),
    )]
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

pub fn catchers() -> Vec<Catcher> {
    errors![
        errors::not_found,
        errors::bad_request,
        errors::forbidden,
        errors::internal_server
    ]
}
