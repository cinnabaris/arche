use chrono::{Datelike, NaiveDate, NaiveDateTime, Utc};

#[derive(Queryable)]
pub struct Member {
    pub id: i64,
    pub nick_name: String,
    pub real_name: String,
    pub gender: String,
    pub birthday: NaiveDate,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub line: Option<String>,
    pub wechat: Option<String>,
    pub skype: Option<String>,
    pub weibo: Option<String>,
    pub facebook: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Member {
    pub fn age(&self) -> i32 {
        Utc::now().year() - self.birthday.year()
    }
}

#[derive(Queryable)]
pub struct Policy {
    pub id: i64,
    pub user_id: i64,
    pub role_id: i64,
    pub nbf: NaiveDate,
    pub exp: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Policy {
    pub fn enable(&self) -> bool {
        let today = Utc::now().naive_utc().date();
        today.ge(&self.nbf) && today.le(&self.exp)
    }
}

#[derive(Queryable)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub resource_type: Option<String>,
    pub resource_id: Option<i64>,
    pub created_at: NaiveDateTime,
}
