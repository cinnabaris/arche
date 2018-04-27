// pub mod c_home;
pub mod c_users;
pub mod models;
pub mod seed;
pub mod workers;

use rocket::Route;

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    vec![
    // ("/", routes!(c_home::index))
    ]
}
