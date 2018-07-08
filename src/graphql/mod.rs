macro_rules! gq {
    ($x:expr, $y:expr) => {
        ge!($y.call(&$x.context()))
    };
}

macro_rules! ge {
    ($x:expr) => {
        match $x {
            Ok(v) => Ok(v),
            Err(e) => Err(juniper::FieldError::new(e, juniper::Value::Null)),
        }
    };
}

pub mod context;
pub mod mutation;
pub mod query;
pub mod schema;

use std::net::SocketAddr;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use juniper_rocket;
use rocket::{response::content::Html, Route, State};

use super::{
    context::Context,
    orm::PooledConnection as Db,
    request::{Home, Locale, Token},
};

pub const UID: &'static str = "uid";
pub const ACT: &'static str = "act";

pub fn routes() -> (&'static str, Vec<Route>) {
    ("/", routes![doc, handler])
}

#[get("/doc")]
fn doc() -> Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[post("/graphql", data = "<request>")]
fn handler(
    db: Db,
    home: Home,
    locale: Locale,
    remote: SocketAddr,
    token: Token,
    request: juniper_rocket::GraphQLRequest,
    schema: State<schema::Schema>,
    ctx: State<Arc<Context>>,
) -> juniper_rocket::GraphQLResponse {
    let Locale(locale) = locale;
    let Home(home) = home;
    let Token(token) = token;

    request.execute(
        &schema,
        &context::Context {
            db: db,
            app: Arc::clone(&ctx),
            home: home,
            locale: locale,
            token: match token {
                Some(t) => Some(t),
                None => None,
            },
            client_ip: format!("{}", remote.ip()),
        },
    )
}

#[derive(Serialize, GraphQLObject, Deserialize, Debug)]
pub struct H {
    created_at: DateTime<Utc>,
}
impl H {
    pub fn new() -> Self {
        Self {
            created_at: Utc::now(),
        }
    }
}
