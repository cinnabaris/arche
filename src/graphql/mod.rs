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

use std::fmt;
// use std::net::SocketAddr;

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum Role {
    Admin,
    Root,
    Member,
    By(String),
}

impl fmt::Display for Role {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::Admin => fmt.write_str("admin"),
            Role::Root => fmt.write_str("root"),
            Role::Member => fmt.write_str("member"),
            Role::By(n) => fmt.write_str(&n),
        }
    }
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

// use juniper_rocket::{graphiql_source, GraphQLRequest, GraphQLResponse};
// use rocket::response::content::Html;
// use rocket::{Route, State};

// use nut::{
//     env::Config, graphql::context::Context, i18n::Locale, jwt::Token, orm::Connection as Database,
//     security::Encryptor,
// };
//
// pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
//     vec![("/", routes!(doc, get))]
// }
//
// #[get("/doc")]
// fn doc() -> Html<String> {
//     graphiql_source("/graphql")
// }
//
// // #[get("/graphql?<request>")]
// #[post("/graphql", data = "<request>")]
// fn get(
//     db: Database,
//     config: Config,
//     encryptor: Encryptor,
//     token: Token,
//     locale: Locale,
//     remote: SocketAddr,
//     request: GraphQLRequest,
//     schema: State<schema::Schema>,
// ) -> GraphQLResponse {
//     let Token(token) = token;
//     request.execute(
//         &schema,
//         &Context {
//             config: config,
//             encryptor: encryptor,
//             locale: locale.name,
//             remote: remote,
//             db: db,
//             token: token,
//         },
//     )
// }
