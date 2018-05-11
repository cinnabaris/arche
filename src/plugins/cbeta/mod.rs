pub mod c_books;
pub mod models;
pub mod schema;
pub mod seed;

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use rocket::Route;
use sitemap::structs::ChangeFreq;

use super::super::{orm::Connection as Db, result::Result, schema::cbeta_books};

pub fn sitemap(db: &Db) -> Result<Vec<(String, f32, ChangeFreq, NaiveDateTime)>> {
    let pri = 0.2;

    let mut items = Vec::new();

    for n in vec!["books", "notes"] {
        items.push((
            format!("{}/{}/", ROOT, n),
            pri,
            ChangeFreq::Weekly,
            Utc::now().naive_utc(),
        ));
    }

    let books: Vec<(i32, NaiveDateTime)> = cbeta_books::dsl::cbeta_books
        .select((cbeta_books::dsl::id, cbeta_books::dsl::updated_at))
        .load(db)?;
    for (id, updated_at) in books {
        items.push((
            format!("{}/books/{}", ROOT, id),
            pri,
            ChangeFreq::Monthly,
            updated_at,
        ));
    }

    Ok(items)
}

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    vec![(ROOT, routes!(c_books::index, c_books::show, c_books::page))]
}

const ROOT: &'static str = "/cbeta";
