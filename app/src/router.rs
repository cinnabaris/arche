use nut;
use rocket::Route;

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    let mut items = Vec::new();

    items.extend_from_slice(nut::router::routes().as_slice());
    // items.extend_from_slice(graphql::routes().as_slice());

    items
}
