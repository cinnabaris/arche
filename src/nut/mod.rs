pub mod c_home;
pub mod c_users;
pub mod models;
pub mod workers;

use rocket::Route;

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    vec![(
        "/api",
        routes!(
            c_home::get_locales,
            c_home::get_layout,
            c_home::post_install
        ),
    )]
}
