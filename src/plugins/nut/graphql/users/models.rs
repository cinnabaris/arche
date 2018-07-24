use std::convert::From;

use chrono::{DateTime, Utc};

use super::super::super::models;

#[derive(GraphQLObject, Debug, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub sign_in_count: String,
    pub last_sign_in_at: Option<DateTime<Utc>>,
    pub last_sign_in_ip: Option<String>,
    pub current_sign_in_at: Option<DateTime<Utc>>,
    pub current_sign_in_ip: Option<String>,
}

#[derive(GraphQLObject, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Policy {
    pub role: String,
    pub resource: Option<String>,
}

impl From<models::Policy> for Policy {
    fn from(it: models::Policy) -> Self {
        Self {
            role: it.role,
            resource: it.resource,
        }
    }
}

#[derive(GraphQLObject, Debug, Serialize)]
pub struct Profile {
    pub name: String,
    pub email: String,
    pub logo: String,
}

#[derive(GraphQLObject, Debug, Serialize)]
pub struct Log {
    pub id: String,
    pub message: String,
    pub ip: String,
    pub created_at: DateTime<Utc>,
}

#[derive(GraphQLObject, Debug, Serialize)]
pub struct SignIn {
    pub token: String,
}
