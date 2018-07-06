use super::super::super::request::Locale;

#[get("/")]
pub fn home(lng: Locale) -> String {
    format!("Hello, arche {:?}!", lng)
}
