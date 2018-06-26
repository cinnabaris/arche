pub mod context;
pub mod mutation;
pub mod query;
pub mod schema;

use std::net::SocketAddr;

use juniper_rocket::{graphiql_source, GraphQLRequest, GraphQLResponse};
use rocket::response::content::Html;
use rocket::{Route, State};

use nut::i18n::Locale;

#[derive(Serialize, GraphQLObject, Deserialize, Debug)]
pub struct H {}

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    vec![("/", routes!(doc, get))]
}

#[get("/doc")]
fn doc() -> Html<String> {
    graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get(
    locale: Locale,
    remote: SocketAddr,
    request: GraphQLRequest,
    schema: State<schema::Schema>,
) -> GraphQLResponse {
    request.execute(
        &schema,
        &context::Context {
            locale: locale.name,
            remote: remote,
        },
    )
}
