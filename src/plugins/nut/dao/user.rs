use std::fmt;

use chrono::Utc;
use diesel::{insert_into, prelude::*, update};
use uuid::Uuid;

use super::super::super::super::{
    errors::Result,
    orm::{schema::users, Connection as Db},
    utils,
};

#[derive(Debug)]
pub enum Type {
    Google,
    Facebook,
    Line,
    Github,
    WeChat,
    Email,
}

impl fmt::Display for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Google => fmt.write_str("google"),
            Type::Facebook => fmt.write_str("facebook"),
            Type::Github => fmt.write_str("github"),
            Type::WeChat => fmt.write_str("wechat"),
            Type::Line => fmt.write_str("line"),
            Type::Email => fmt.write_str("email"),
        }
    }
}

pub fn add_by_email(
    db: &Db,
    name: &String,
    email: &String,
    password: &String,
) -> Result<Option<i64>> {
    let now = Utc::now().naive_utc();
    match users::dsl::users
        .select(users::dsl::id)
        .filter(users::dsl::email.eq(email))
        .first::<i64>(db)
    {
        Ok(_) => Ok(None),
        Err(_) => {
            let password = utils::hash::sum(password.as_bytes())?;
            let id = insert_into(users::dsl::users)
                .values((
                    users::dsl::email.eq(email),
                    users::dsl::name.eq(name),
                    users::dsl::password.eq(&Some(password)),
                    users::dsl::provider_type.eq(&format!("{}", Type::Email)),
                    users::dsl::provider_id.eq(email),
                    users::dsl::uid.eq(&Uuid::new_v4().to_string()),
                    users::dsl::sign_in_count.eq(0),
                    users::dsl::updated_at.eq(&now),
                    users::dsl::created_at.eq(&now),
                ))
                .returning(users::dsl::id)
                .get_result::<i64>(db)?;
            Ok(Some(id))
        }
    }
}
