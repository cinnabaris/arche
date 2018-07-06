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

use chrono::{DateTime, Utc};
use juniper_rocket;
use rocket::{response::content::Html, Route, State};

use super::{
    env::Config,
    orm::PooledConnection as Db,
    request::{Home, Locale, Token},
    utils::Encryptor,
};

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
    config: State<Config>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<schema::Schema>,
    encryptor: State<Encryptor>,
) -> juniper_rocket::GraphQLResponse {
    let Locale(locale) = locale;
    let Home(home) = home;
    let Token(token) = token;

    request.execute(
        &schema,
        &context::Context {
            db: db,
            home: home,
            locale: locale,
            config: config.clone(),
            encryptor: encryptor.clone(),
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
