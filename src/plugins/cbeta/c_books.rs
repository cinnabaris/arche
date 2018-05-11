use std::ops::Deref;
use std::path::PathBuf;

use rocket_contrib::Template;
use serde_json::Value;

use super::super::super::{
    format::{Pager, Pagination},
    i18n::{self, Locale},
    orm::PooledConnection as Db,
    result::{Error, Result},
};
use super::super::nut::application_layout;
use super::models::{Book, Page};

#[get("/books?<pager>")]
fn index(db: Db, lng: Locale, pager: Option<Pager>) -> Result<Template> {
    let pager = match pager {
        Some(pager) => pager,
        None => Pager::new(1, 60),
    };
    let db = db.deref();
    let mut lyt = application_layout(
        db,
        &lng.name,
        &i18n::t(db, &lng.name, &s!("cbeta.index.title"), None::<Value>),
    )?;

    let (total, items) = Book::list(db, &pager)?;
    lyt.insert(s!("books"), json!(Pagination::new(total, &pager, items)));

    Ok(Template::render("cbeta/books/index", lyt))
}

#[get("/books/<id>")]
fn show(db: Db, id: i32) -> Result<Page> {
    let db = db.deref();
    let page = Page::home(db, &id)?;
    Ok(page)
}

#[get("/books/<id>/<file..>")]
fn page(db: Db, id: i32, file: PathBuf) -> Result<Page> {
    let db = db.deref();

    let page = match file.to_str() {
        Some(file) => Page::get(db, &id, &s!(file)),
        None => Err(Error::WithDescription(format!("bad file path {:?}", file))),
    }?;

    Ok(page)
}
