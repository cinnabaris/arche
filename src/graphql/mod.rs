pub mod context;
pub mod mutation;
pub mod query;
pub mod schema;

use juniper_rocket::{graphiql_source, GraphQLRequest, GraphQLResponse};
use rocket::response::content::Html;
use rocket::{Route, State};

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    vec![("/", routes!(doc, query, mutation))]
}

#[get("/doc")]
fn doc() -> Html<String> {
    graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn query(
    context: State<context::Context>,
    request: GraphQLRequest,
    schema: State<schema::Schema>,
) -> GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn mutation(
    context: State<context::Context>,
    request: GraphQLRequest,
    schema: State<schema::Schema>,
) -> GraphQLResponse {
    request.execute(&schema, &context)
}
