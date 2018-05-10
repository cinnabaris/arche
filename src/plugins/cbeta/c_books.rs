use std::ops::Deref;
use std::path::PathBuf;

use rocket::http::RawStr;
use rocket_contrib::Template;
use serde_json::Value;

use super::super::super::{
    i18n::{self, Locale},
    orm::PooledConnection as Db,
    result::Result,
};
use super::super::nut::application_layout;

#[get("/books")]
fn index(db: Db, lng: Locale) -> Result<Template> {
    let db = db.deref();
    let lyt = application_layout(
        db,
        &lng.name,
        &i18n::t(db, &lng.name, &s!("cbeta.index.title"), None::<Value>),
    )?;
    Ok(Template::render("cbeta/index", lyt))
}

#[get("/books/<uid>")]
fn show(uid: &RawStr) -> Result<String> {
    Ok(s!(uid))
}

#[get("/books/<uid>/<file..>")]
fn page(uid: &RawStr, file: PathBuf) -> Result<Vec<u8>> {
    println!("{:?} {:?}", uid, file);
    let out = Vec::new();
    Ok(out)
}
