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
pub mod routes;
pub mod schema;

// use std::net::SocketAddr;

use chrono::{DateTime, Utc};

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
