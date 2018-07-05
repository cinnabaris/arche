use std::path::{Path, PathBuf};

use rocket::{http::Status, response::NamedFile, Catcher, Route, State};

use super::{env::Config, errors::Result, plugins::nut};

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    let mut items = Vec::new();
    items.push(nut::routes());
    items.push((
        "/",
        routes![
            third,
            upload,
            global,
            assets,
            robots_txt,
            sitemap_xml_gz,
            rss_atom
        ],
    ));
    items
}

pub fn catchers() -> Vec<Catcher> {
    errors![not_found, forbidden, internal_server_error]
}

#[get("/global/<file..>")]
fn global(file: PathBuf) -> Result<NamedFile> {
    Ok(NamedFile::open(
        Path::new("themes").join("global").join(file),
    )?)
}

#[get("/assets/<file..>")]
fn assets(file: PathBuf, cfg: State<Config>) -> Result<NamedFile> {
    Ok(NamedFile::open(
        Path::new("themes")
            .join(cfg.http.theme.clone())
            .join("assets")
            .join(file),
    )?)
}

#[get("/3rd/<file..>")]
fn third(file: PathBuf) -> Result<NamedFile> {
    Ok(NamedFile::open(Path::new("node_modules").join(file))?)
}

#[get("/upload/<file..>")]
fn upload(file: PathBuf) -> Result<NamedFile> {
    Ok(NamedFile::open(Path::new("tmp").join("upload").join(file))?)
}

#[get("/robots.txt")]
fn robots_txt() -> &'static str {
    "robots"
}

#[get("/sitemap.xml.gz")]
fn sitemap_xml_gz() -> &'static str {
    "sitemap"
}

#[get("/rss/<lang>")]
fn rss_atom(lang: String) -> &'static str {
    "rss"
}

#[error(404)]
fn not_found() -> &'static str {
    Status::NotFound.reason
}

#[error(403)]
fn forbidden() -> &'static str {
    Status::Forbidden.reason
}

#[error(500)]
fn internal_server_error() -> &'static str {
    Status::InternalServerError.reason
}
