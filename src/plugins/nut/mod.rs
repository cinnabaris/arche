pub mod forms;
// pub mod models;
// pub mod seed;

use rocket::Route;

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    vec![
    // ("/", routes!(c_home::index))
    ]
}
