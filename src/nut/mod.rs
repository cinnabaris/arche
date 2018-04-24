pub mod c_home;
pub mod c_users;
pub mod models;
pub mod seed;
pub mod workers;

use rocket::Route;

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    vec![
        ("/", routes!(c_home::index)),
        ("/api", routes!(c_home::get_locales, c_home::get_layout)),
        (
            "/api/users",
            routes!(
                c_users::post_sign_up,
                c_users::get_confirm_token,
                c_users::post_confirm,
                c_users::get_unlock_token,
                c_users::post_unlock,
                c_users::post_forgot_password,
                c_users::post_reset_password
            ),
        ),
    ]
}
