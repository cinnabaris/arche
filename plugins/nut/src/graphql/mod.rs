pub mod context;
pub mod models;
pub mod mutation;
pub mod query;

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
