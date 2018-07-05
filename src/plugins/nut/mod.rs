pub mod consumers;
pub mod controllers;
pub mod dao;
pub mod graphql;

use rocket::Route;

pub fn routes() -> (&'static str, Vec<Route>) {
    ("/", routes![controllers::home])
}
