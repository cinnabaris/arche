use std::fmt;
use std::str::FromStr;

use chrono::{Datelike, NaiveDate, NaiveDateTime, Utc};

use super::super::super::errors::Error;

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
    pub role: String,
    pub resource: Option<String>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
    Root,
    Admin,
    Manager,
    Member,
    By(String),
}

impl fmt::Display for Role {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::Root => write!(fmt, "root"),
            Role::Admin => write!(fmt, "admin"),
            Role::Manager => write!(fmt, "manager"),
            Role::Member => write!(fmt, "member"),
            Role::By(n) => write!(fmt, "{}", &n),
        }
    }
}

impl FromStr for Role {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "root" => Role::Root,
            "admin" => Role::Admin,
            "manager" => Role::Manager,
            "member" => Role::Member,
            v => Role::By(v.to_string()),
        })
    }
}
