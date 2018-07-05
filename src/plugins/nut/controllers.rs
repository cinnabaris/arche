#[get("/")]
pub fn home() -> &'static str {
    "Hello, arche!"
}
