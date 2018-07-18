use chrono::{DateTime, Utc};

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
