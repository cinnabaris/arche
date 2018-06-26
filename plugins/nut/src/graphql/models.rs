use std::fmt;

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

#[derive(GraphQLObject, Debug, Serialize)]
#[graphql(description = "Message for translate")]
pub struct Locale {
    pub id: String,
    pub lang: String,
    pub code: String,
    pub message: String,
    pub updated_at: DateTime<Utc>,
}
