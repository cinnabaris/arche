use std::fmt;

use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use hex;
use md5::{self, Digest};
use uuid::Uuid;

use super::super::super::super::{
    errors::Result,
    orm::{schema::users, Connection as Db},
    utils,
};

pub fn sign_in(db: &Db, id: &i64, ip: &String) -> Result<()> {
    let now = Utc::now().naive_utc();
    let it = users::dsl::users.filter(users::dsl::id.eq(id));
    let (current_sign_in_at, current_sign_in_ip, sign_in_count) = users::dsl::users
        .select((
            users::dsl::current_sign_in_at,
            users::dsl::current_sign_in_ip,
            users::dsl::sign_in_count,
        ))
        .filter(users::dsl::id.eq(id))
        .first::<(Option<NaiveDateTime>, Option<String>, i64)>(db)?;
    update(it)
        .set((
            users::dsl::current_sign_in_at.eq(&now),
            users::dsl::current_sign_in_ip.eq(ip),
            users::dsl::last_sign_in_at.eq(&current_sign_in_at),
            users::dsl::last_sign_in_ip.eq(&current_sign_in_ip),
            users::dsl::sign_in_count.eq(&(sign_in_count + 1)),
            users::dsl::updated_at.eq(&now),
        ))
        .execute(db)?;
    Ok(())
}

// https://en.gravatar.com/site/implement/hash/
pub fn gravatar_logo(email: &String) -> String {
    let mut h = md5::Md5::new();
    h.input(email.to_lowercase().trim().as_bytes());
    format!(
        "https://www.gravatar.com/avatar/{}.png",
        hex::encode(h.result().as_slice())
    )
}

pub fn lock(db: &Db, id: &i64, un: bool) -> Result<()> {
    let now = Utc::now().naive_utc();
    let it = users::dsl::users.filter(users::dsl::id.eq(id));
    update(it)
        .set((
            users::dsl::locked_at.eq(&if un { None } else { Some(now) }),
            users::dsl::updated_at.eq(&now),
        ))
        .execute(db)?;
    Ok(())
}

pub fn confirm(db: &Db, id: &i64) -> Result<()> {
    let now = Utc::now().naive_utc();
    let it = users::dsl::users.filter(users::dsl::id.eq(id));
    update(it)
        .set((
            users::dsl::confirmed_at.eq(&Some(now)),
            users::dsl::updated_at.eq(&now),
        ))
        .execute(db)?;
    Ok(())
}

pub fn count(db: &Db) -> Result<i64> {
    let cnt: i64 = users::dsl::users.count().get_result(db)?;
    Ok(cnt)
}

pub fn is_email_exist(db: &Db, email: &String) -> Result<bool> {
    let cnt: i64 = users::dsl::users
        .filter(users::dsl::email.eq(email))
        .count()
        .get_result(db)?;
    Ok(cnt > 0)
}

pub fn set_password(db: &Db, id: &i64, password: &String) -> Result<()> {
    let now = Utc::now().naive_utc();
    let password = utils::hash::sum(password.as_bytes())?;
    let it = users::dsl::users.filter(users::dsl::id.eq(id));
    update(it)
        .set((
            users::dsl::password.eq(&password),
            users::dsl::updated_at.eq(&now),
        ))
        .execute(db)?;
    Ok(())
}

pub fn add_by_email(
    db: &Db,
    name: &String,
    email: &String,
    password: &String,
) -> Result<(i64, String)> {
    let now = Utc::now().naive_utc();
    let uid = Uuid::new_v4().to_string();

    let password = utils::hash::sum(password.as_bytes())?;
    let id = insert_into(users::dsl::users)
        .values((
            users::dsl::email.eq(email),
            users::dsl::name.eq(name),
            users::dsl::password.eq(&Some(password)),
            users::dsl::provider_type.eq(&format!("{}", Type::Email)),
            users::dsl::provider_id.eq(email),
            users::dsl::uid.eq(&uid),
            users::dsl::logo.eq(&gravatar_logo(email)),
            users::dsl::sign_in_count.eq(0),
            users::dsl::updated_at.eq(&now),
            users::dsl::created_at.eq(&now),
        ))
        .returning(users::dsl::id)
        .get_result::<i64>(db)?;
    Ok((id, uid))
}

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
