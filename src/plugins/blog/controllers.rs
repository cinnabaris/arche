use std::ops::Deref;
use std::path::PathBuf;

use rocket::response::{self, NamedFile, Responder};
use rocket::Request;
use rocket_contrib::Template;
use serde_json::Value;

use super::super::super::{
    i18n::{self, Locale},
    orm::PooledConnection as Db,
    result::Result,
};
use super::super::nut::application_layout;
use super::seed;

#[get("/")]
fn index(db: Db, lng: Locale) -> Result<Template> {
    let db = db.deref();
    let mut lyt = application_layout(
        &lng.name,
        &i18n::t(db, &lng.name, &s!("blog.index.title"), None::<Value>),
    )?;
    lyt.insert(s!("items"), json!(seed::files()?));
    Ok(Template::render("blog/index", lyt))
}

#[get("/<file..>")]
fn show<'r>(lng: Locale, file: PathBuf) -> Result<Blog> {
    let pt = file.as_path();

    if let Some(ext) = file.extension() {
        if let Some(ext) = ext.to_str() {
            if ext == seed::Markdown::EXT {
                let (title, body) = seed::Markdown::read(pt)?;

                let mut lyt = application_layout(&lng.name, &title)?;
                lyt.insert(s!("body"), json!(body));
                return Ok(Blog::Html(Template::render("blog/show", lyt)));
            }
        }
    }
    Ok(Blog::File(NamedFile::open(seed::path(Some(pt)))?))
}

enum Blog {
    File(NamedFile),
    Html(Template),
}

impl<'r> Responder<'r> for Blog {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        match self {
            Blog::File(f) => f.respond_to(req),
            Blog::Html(t) => t.respond_to(req),
        }
    }
}
