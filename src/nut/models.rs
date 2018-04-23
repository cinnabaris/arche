use std::net::SocketAddr;
use std::ops::Add;
use std::ops::Deref;

use chrono::{Duration, NaiveDate, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::{delete, insert_into, update};
use hex;
use md5::{self, Digest};
use serde::ser::Serialize;
use uuid::Uuid;

use super::super::i18n::Locale;
use super::super::orm::Connection as Db;
use super::super::result::Result;
use super::super::schema::{logs, policies, roles, users};
use super::super::security::hash;

//-----------------------------------------------------------------------------

pub const ROLE_ADMIN: &'static str = "admin";
pub const ROLE_ROOT: &'static str = "root";

// ----------------------------------------------------------------------------

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub uid: String,
    pub password: Option<Vec<u8>>,
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "providerType")]
    pub provider_type: String,
    pub logo: Option<String>,
    #[serde(rename = "signInCount")]
    pub sign_in_count: i64,
    #[serde(rename = "currentSignInAt")]
    pub current_sign_in_at: Option<NaiveDateTime>,
    #[serde(rename = "currentSignInIp")]
    pub current_sign_in_ip: Option<String>,
    #[serde(rename = "lastSignInat")]
    pub last_sign_in_at: Option<NaiveDateTime>,
    #[serde(rename = "lastSignInIp")]
    pub last_sign_in_ip: Option<String>,
    #[serde(rename = "confirmedAt")]
    pub confirmed_at: Option<NaiveDateTime>,
    #[serde(rename = "lockedAt")]
    pub locked_at: Option<NaiveDateTime>,
    #[serde(rename = "updatedAt")]
    pub updated_at: NaiveDateTime,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
}

impl User {
    // https://en.gravatar.com/site/implement/hash/
    pub fn gravatar_logo(email: &String) -> String {
        let mut h = md5::Md5::new();
        h.input(email.to_lowercase().trim().as_bytes());
        format!(
            "https://www.gravatar.com/avatar/{}.png",
            hex::encode(h.result().as_slice())
        )
    }
    pub fn get_by_email(db: &Db, email: &String) -> Result<User> {
        let it = users::dsl::users
            .filter(users::dsl::email.eq(email))
            .first::<User>(db.deref())?;
        Ok(it)
    }
    pub fn get_by_uid(db: &Db, uid: &String) -> Result<User> {
        let it = users::dsl::users
            .filter(users::dsl::uid.eq(uid))
            .first::<User>(db.deref())?;
        Ok(it)
    }
    pub fn count(db: &Db) -> Result<i64> {
        let cnt: i64 = users::dsl::users.count().get_result(db.deref())?;
        Ok(cnt)
    }
    pub fn is_email_exist(db: &Db, email: &String) -> Result<bool> {
        let cnt: i64 = users::dsl::users
            .filter(users::dsl::email.eq(email))
            .count()
            .get_result(db.deref())?;
        Ok(cnt > 0)
    }
    pub fn add_by_email(db: &Db, name: &String, email: &String, password: &String) -> Result<i64> {
        let it: User = insert_into(users::dsl::users)
            .values((
                users::dsl::name.eq(name),
                users::dsl::email.eq(email),
                users::dsl::password.eq(&hash::sum(password.as_bytes())?),
                users::dsl::logo.eq(&User::gravatar_logo(email)),
                users::dsl::uid.eq(&Uuid::new_v4().to_string()),
                users::dsl::provider_id.eq(email),
                users::dsl::provider_type.eq("email"),
                users::dsl::updated_at.eq(Utc::now().naive_utc()),
            ))
            .get_result(db.deref())?;
        Ok(it.id)
    }
    pub fn sign_in(
        db: &Db,
        user: &i64,
        remote: &SocketAddr,
        sign_in_count: i64,
        current_sign_in_ip: Option<String>,
        current_sign_in_at: Option<NaiveDateTime>,
    ) -> Result<()> {
        let ip = format!("{}", remote.ip());
        let it = users::dsl::users.filter(users::dsl::id.eq(user));
        let now = Utc::now().naive_utc();
        update(it)
            .set((
                users::dsl::sign_in_count.eq(sign_in_count + 1),
                users::dsl::last_sign_in_ip.eq(current_sign_in_ip),
                users::dsl::last_sign_in_at.eq(current_sign_in_at),
                users::dsl::current_sign_in_ip.eq(ip),
                users::dsl::current_sign_in_at.eq(&now),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(db.deref())?;
        Ok(())
    }
    pub fn confirm(db: &Db, user: &i64) -> Result<()> {
        let it = users::dsl::users.filter(users::dsl::id.eq(user));
        let now = Utc::now().naive_utc();
        update(it)
            .set((
                users::dsl::confirmed_at.eq(&now),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(db.deref())?;
        Ok(())
    }

    pub fn lock(db: &Db, user: &i64, ok: bool) -> Result<()> {
        let it = users::dsl::users.filter(users::dsl::id.eq(user));
        let now = Utc::now().naive_utc();
        update(it)
            .set((
                users::dsl::locked_at.eq(if ok { Some(now) } else { None }),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(db.deref())?;
        Ok(())
    }

    pub fn password(db: &Db, user: &i64, password: &String) -> Result<()> {
        let it = users::dsl::users.filter(users::dsl::id.eq(user));
        update(it)
            .set((
                users::dsl::password.eq(&hash::sum(password.as_bytes())?),
                users::dsl::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(db.deref())?;
        Ok(())
    }
}

// ----------------------------------------------------------------------------

#[derive(Serialize, Queryable, Deserialize, Debug, Clone)]
pub struct Log {
    pub id: i64,
    #[serde(rename = "userId")]
    pub user_id: i64,
    pub message: String,
    pub ip: String,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
}

impl Log {
    pub fn list(db: &Db, user: &i64) -> Result<Vec<Log>> {
        let it = logs::dsl::logs
            .filter(logs::dsl::user_id.eq(user))
            .get_results::<Log>(db.deref())?;
        Ok(it)
    }
    pub fn add<S: Serialize>(
        db: &Db,
        user: &i64,
        lang: &String,
        remote: &SocketAddr,
        code: &String,
        args: Option<S>,
    ) -> Result<()> {
        let ip = format!("{}", remote.ip());
        let message = Locale::t(db, lang, code, args);

        insert_into(logs::dsl::logs)
            .values((
                logs::dsl::user_id.eq(user),
                logs::dsl::ip.eq(ip),
                logs::dsl::message.eq(&message),
            ))
            .execute(db.deref())?;
        Ok(())
    }
}

// ----------------------------------------------------------------------------

#[derive(Serialize, Queryable, Deserialize, Debug, Clone)]
pub struct Role {
    pub id: i64,
    pub name: String,
    #[serde(rename = "resourceId")]
    pub resource_id: Option<i64>,
    #[serde(rename = "resourceType")]
    pub resource_type: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: NaiveDateTime,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
}

impl Role {
    pub fn get(
        db: &Db,
        name: &String,
        resource_type: &Option<String>,
        resource_id: &Option<i64>,
    ) -> Result<i64> {
        match roles::dsl::roles
            .select(roles::dsl::id)
            .filter(roles::dsl::name.eq(name))
            .filter(roles::dsl::resource_type.eq(resource_type))
            .filter(roles::dsl::resource_id.eq(resource_id))
            .first::<i64>(db.deref())
        {
            Ok(id) => Ok(id),
            Err(_) => {
                let it: Role = insert_into(roles::dsl::roles)
                    .values((
                        roles::dsl::name.eq(name),
                        roles::dsl::resource_type.eq(resource_type),
                        roles::dsl::resource_id.eq(resource_id),
                        roles::dsl::updated_at.eq(Utc::now().naive_utc()),
                    ))
                    .get_result(db.deref())?;
                Ok(it.id)
            }
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Serialize, Queryable, Deserialize, Debug, Clone)]
pub struct Policy {
    pub id: i64,
    #[serde(rename = "userId")]
    pub user_id: i64,
    #[serde(rename = "roleId")]
    pub role_id: i64,
    pub nbf: NaiveDate,
    pub exp: NaiveDate,
    #[serde(rename = "updatedAt")]
    pub updated_at: NaiveDateTime,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
}

impl Policy {
    pub fn can(db: &Db, user: &i64, role: &i64) -> bool {
        if let Ok((nbf, exp)) = policies::dsl::policies
            .select((policies::dsl::nbf, policies::dsl::exp))
            .filter(policies::dsl::user_id.eq(user))
            .filter(policies::dsl::role_id.eq(role))
            .first::<(NaiveDate, NaiveDate)>(db.deref())
        {
            let today = Utc::now().date().naive_utc();
            return nbf <= today && today <= exp;
        }
        false
    }

    pub fn apply(db: &Db, user: &i64, role: &i64, days: i64) -> Result<i64> {
        let nbf = Utc::now();
        let exp = nbf.add(Duration::days(days));
        let nbf = nbf.date().naive_utc();
        let exp = exp.date().naive_utc();

        let now = Utc::now().naive_utc();
        match policies::dsl::policies
            .select(policies::dsl::id)
            .filter(policies::dsl::user_id.eq(user))
            .filter(policies::dsl::role_id.eq(role))
            .first::<i64>(db.deref())
        {
            Ok(id) => {
                let it = policies::dsl::policies.filter(policies::dsl::id.eq(&id));
                update(it)
                    .set((
                        policies::dsl::nbf.eq(&nbf),
                        policies::dsl::exp.eq(&exp),
                        policies::dsl::updated_at.eq(&now),
                    ))
                    .execute(db.deref())?;
                Ok(id)
            }
            Err(_) => {
                let it: Policy = insert_into(policies::dsl::policies)
                    .values((
                        policies::dsl::role_id.eq(role),
                        policies::dsl::user_id.eq(user),
                        policies::dsl::nbf.eq(&nbf),
                        policies::dsl::exp.eq(&exp),
                        policies::dsl::updated_at.eq(&now),
                    ))
                    .get_result(db.deref())?;
                Ok(it.id)
            }
        }
    }

    pub fn deny(db: &Db, user: &i64, role: &i64) -> Result<usize> {
        let it = policies::dsl::policies
            .filter(policies::dsl::user_id.eq(user))
            .filter(policies::dsl::role_id.eq(role));

        let num = delete(it).execute(db.deref())?;
        Ok(num)
    }
}
