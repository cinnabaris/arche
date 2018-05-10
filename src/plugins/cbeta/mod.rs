pub mod c_books;
pub mod models;
pub mod seed;

use chrono::{NaiveDateTime, Utc};
use rocket::Route;
use sitemap::structs::ChangeFreq;

use super::super::result::Result;

pub fn sitemap() -> Result<Vec<(String, f32, ChangeFreq, NaiveDateTime)>> {
    let pri = 0.1;

    let mut items = Vec::new();
    items.push((s!(ROOT), pri, ChangeFreq::Weekly, Utc::now().naive_utc()));

    Ok(items)
}

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    vec![(ROOT, routes!(c_books::index, c_books::show, c_books::page))]
}

const ROOT: &'static str = "/cbeta";
