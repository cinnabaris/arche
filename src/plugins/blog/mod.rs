pub mod controllers;
pub mod seed;

use chrono::{NaiveDateTime, Utc};
use rocket::Route;
use sitemap::structs::ChangeFreq;

use super::super::result::Result;

pub fn sitemap() -> Result<Vec<(String, f32, ChangeFreq, NaiveDateTime)>> {
    let pri = 0.1;

    let mut items = Vec::new();
    items.push((s!(ROOT), pri, ChangeFreq::Weekly, Utc::now().naive_utc()));
    for it in seed::files()? {
        items.push((
            format!("{}/{}", ROOT, it.uri),
            pri,
            ChangeFreq::Weekly,
            it.updated_at,
        ));
    }
    Ok(items)
}

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    vec![(ROOT, routes!(controllers::index))]
}

pub fn rss(_lang: &String) -> Vec<(String, String, String, NaiveDateTime)> {
    vec![(
        s!("/blog/readme.md"),
        s!("README"),
        s!("CONTENT"),
        Utc::now().naive_utc(),
    )]
}

const ROOT: &'static str = "/blog";
