use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Topic {
    pub id: i64,
    pub user_id: i64,
    pub member_id: i64,
    pub tag: String,
    pub name: String,
    pub gender: String,
    pub age: i16,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub reason: String,
    pub media_type: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable)]
pub struct Post {
    pub id: i64,
    pub topic_id: i64,
    pub user_id: i64,
    pub method: String,
    pub body: String,
    pub media_type: String,
    pub begin: NaiveDateTime,
    pub end: NaiveDateTime,
    pub create_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
