pub mod consumers;
pub mod controllers;
pub mod models;
pub mod schema;
pub mod seed;

use std::collections::HashMap;

use rocket::Route;
use serde_json::Value;

use super::super::{i18n, orm::Connection as Db, result::Result};

pub fn application_layout(
    db: &Db,
    lang: &String,
    title: &String,
) -> Result<HashMap<String, Value>> {
    let mut out = HashMap::new();
    out.insert(s!("locale"), json!(lang));
    out.insert(s!("title"), json!(title));

    // site info
    let mut site = HashMap::new();
    for k in vec!["title", "subhead", "keywords", "description", "copyright"] {
        site.insert(
            s!(k),
            i18n::t(db, lang, &format!("site.{}", k), None::<Value>),
        );
    }
    out.insert(s!("site"), json!(site));

    Ok(out)
}

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    vec![("/", routes!(controllers::index,))]
}
