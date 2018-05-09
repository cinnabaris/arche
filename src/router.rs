use std::path::{Path, PathBuf};

use rocket::http::Status;
use rocket::response::NamedFile;
use rocket::{self, Catcher, Route, State};

use super::{env, graphql, plugins};

pub fn catchers() -> Vec<Catcher> {
    errors![not_found, bad_request, forbidden, internal_server]
}

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    let mut items = Vec::new();
    items.extend_from_slice(plugins::nut::routes().as_slice());
    items.extend_from_slice(graphql::routes().as_slice());
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
    Status::NotFound.reason
}

#[error(400)]
fn bad_request() -> &'static str {
    Status::BadRequest.reason
}

#[error(403)]
fn forbidden() -> &'static str {
    Status::Forbidden.reason
}

#[error(500)]
fn internal_server() -> &'static str {
    Status::InternalServerError.reason
}
