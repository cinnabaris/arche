use std::ops::Deref;

use rocket_contrib::Template;
use serde_json::Value;

use super::super::super::{
    i18n::{self, Locale},
    orm::PooledConnection as Db,
    result::Result,
};
use super::application_layout;

#[get("/")]
fn index(db: Db, lng: Locale) -> Result<Template> {
    let db = db.deref();
    let lyt = application_layout(
        db,
        &lng.name,
        &i18n::t(db, &lng.name, &s!("nut.index.title"), None::<Value>),
    )?;
    Ok(Template::render("index", lyt))
}
