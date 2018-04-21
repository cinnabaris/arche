use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket::{self, Catcher, Route, State};

use super::{env, nut};

pub fn catchers() -> Vec<Catcher> {
    errors![not_found, bad_request]
}

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    let mut items = Vec::new();
    items.extend_from_slice(nut::routes().as_slice());
    return items;
}

//-----------------------------------------------------------------------------

#[get("/assets/<file..>")]
pub fn get_assets(cfg: State<env::Config>, file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(
        Path::new("themes")
            .join(&cfg.http.theme)
            .join("assets")
            .join(file),
    ).ok()
}

#[error(404)]
fn not_found() -> &'static str {
    "Not found"
}

#[error(400)]
fn bad_request() -> &'static str {
    "Bad request"
}
