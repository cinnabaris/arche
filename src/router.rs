use rocket::{self, Catcher, Route};

use super::nut;

pub fn catchers() -> Vec<Catcher> {
    errors![not_found, bad_request]
}

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    let mut items = Vec::new();
    items.extend_from_slice(nut::routes().as_slice());
    return items;
}

//-----------------------------------------------------------------------------

#[error(404)]
fn not_found() -> &'static str {
    "Not found"
}

#[error(400)]
fn bad_request() -> &'static str {
    "Bad request"
}
