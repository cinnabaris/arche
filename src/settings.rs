use chrono::{DateTime, Utc};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Setting {
    pub id: i64,
    pub key: String,
    pub value: Vec<u8>,
    pub salt: Option<Vec<u8>>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}
