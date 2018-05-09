pub mod models;
pub mod schema;
pub mod seed;

use rocket::Route;

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    vec![
    // ("/", routes!(c_home::index))
    ]
}
