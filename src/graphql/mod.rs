pub mod context;
pub mod mutation;
pub mod query;
pub mod schema;

use std::net::SocketAddr;

use juniper_rocket::{graphiql_source, GraphQLRequest, GraphQLResponse};
use rocket::response::content::Html;
use rocket::{Route, State};

use super::i18n::Lang;
use super::orm::Connection;
use super::spree::guards::CurrentUser;

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    vec![("/", routes!(doc, get, post))]
}

#[get("/doc")]
fn doc() -> Html<String> {
    graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get(
    db: Connection,
    lang: Lang,
    remote: SocketAddr,
    user: Option<CurrentUser>,
    request: GraphQLRequest,
    schema: State<schema::Schema>,
) -> GraphQLResponse {
    execute(db, lang, remote, user, request, schema)
}

#[post("/graphql", data = "<request>")]
fn post(
    db: Connection,
    lang: Lang,
    remote: SocketAddr,
    user: Option<CurrentUser>,
    request: GraphQLRequest,
    schema: State<schema::Schema>,
) -> GraphQLResponse {
    execute(db, lang, remote, user, request, schema)
}

fn execute(
    db: Connection,
    lang: Lang,
    remote: SocketAddr,
    user: Option<CurrentUser>,
    request: GraphQLRequest,
    schema: State<schema::Schema>,
) -> GraphQLResponse {
    let Lang(lang) = lang;
    request.execute(
        &schema,
        &context::Context {
            db: db,
            locale: lang,
            user: user,
            remote: remote,
        },
    )
}
