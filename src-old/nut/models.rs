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

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Policy {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub nbf: NaiveDate,
    pub exp: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Policy {
    pub fn users(db: &Db, role: &i32) -> Result<Vec<i32>> {
        let db = db.deref();
        let ids = policies::dsl::policies
            .select((
                policies::dsl::user_id,
                policies::dsl::nbf,
                policies::dsl::exp,
            ))
            .filter(policies::dsl::role_id.eq(role))
            .get_results::<(i32, NaiveDate, NaiveDate)>(db)?;

        let today = Utc::now().date().naive_utc();
        let mut items = Vec::new();
        for (id, nbf, exp) in ids {
            if nbf <= today && today <= exp {
                items.push(id);
            }
        }
        Ok(items)
    }

    pub fn roles(db: &Db, user: &i32) -> Result<Vec<i32>> {
        let db = db.deref();
        let ids = policies::dsl::policies
            .select((
                policies::dsl::role_id,
                policies::dsl::nbf,
                policies::dsl::exp,
            ))
            .filter(policies::dsl::user_id.eq(user))
            .get_results::<(i32, NaiveDate, NaiveDate)>(db)?;

        let today = Utc::now().date().naive_utc();
        let mut items = Vec::new();
        for (id, nbf, exp) in ids {
            if nbf <= today && today <= exp {
                items.push(id);
            }
        }
        Ok(items)
    }

    pub fn can(db: &Db, user: &i32, role: &i32) -> bool {
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

    pub fn apply(db: &Db, user: &i32, role: &i32, ttl: Duration) -> Result<()> {
        let db = db.deref();
        let nbf = Utc::now();
        let exp = nbf.add(ttl);
        let nbf = nbf.date().naive_utc();
        let exp = exp.date().naive_utc();

        let now = Utc::now().naive_utc();
        match policies::dsl::policies
            .select(policies::dsl::id)
            .filter(policies::dsl::user_id.eq(user))
            .filter(policies::dsl::role_id.eq(role))
            .first::<i32>(db)
        {
            Ok(id) => {
                let it = policies::dsl::policies.filter(policies::dsl::id.eq(&id));
                update(it)
                    .set((
                        policies::dsl::nbf.eq(&nbf),
                        policies::dsl::exp.eq(&exp),
                        policies::dsl::updated_at.eq(&now),
                    ))
                    .execute(db)?;
                Ok(())
            }
            Err(_) => {
                insert_into(policies::dsl::policies)
                    .values((
                        policies::dsl::role_id.eq(role),
                        policies::dsl::user_id.eq(user),
                        policies::dsl::nbf.eq(&nbf),
                        policies::dsl::exp.eq(&exp),
                        policies::dsl::updated_at.eq(&now),
                    ))
                    .execute(db)?;
                Ok(())
            }
        }
    }

    pub fn deny(db: &Db, user: &i32, role: &i32) -> Result<usize> {
        let it = policies::dsl::policies
            .filter(policies::dsl::user_id.eq(user))
            .filter(policies::dsl::role_id.eq(role));

        let num = delete(it).execute(db.deref())?;
        Ok(num)
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub resource_id: Option<i32>,
    pub resource_type: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Role {
    pub const ADMIN: &'static str = "admin";
    pub const ROOT: &'static str = "root";
    pub const MEMBER: &'static str = "member";

    pub fn by_id(db: &Db, id: &i32) -> Result<Role> {
        Ok(roles::dsl::roles
            .filter(roles::dsl::id.eq(id))
            .first::<Role>(db.deref())?)
    }

    pub fn get(
        db: &Db,
        name: &String,
        resource_type: &Option<String>,
        resource_id: &Option<i32>,
    ) -> Result<i32> {
        let db = db.deref();
        let now = Utc::now().naive_utc();
        match roles::dsl::roles
            .select(roles::dsl::id)
            .filter(roles::dsl::name.eq(name))
            .filter(roles::dsl::resource_type.eq(resource_type))
            .filter(roles::dsl::resource_id.eq(resource_id))
            .first::<i32>(db)
        {
            Ok(id) => Ok(id),
            Err(_) => {
                let id = insert_into(roles::dsl::roles)
                    .values((
                        roles::dsl::name.eq(name),
                        roles::dsl::resource_type.eq(resource_type),
                        roles::dsl::resource_id.eq(resource_id),
                        roles::dsl::updated_at.eq(&now),
                    ))
                    .returning(roles::dsl::id)
                    .get_result::<i32>(db)?;
                Ok(id)
            }
        }
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub uid: String,
    pub password: Option<Vec<u8>>,
    pub provider_id: String,
    pub provider_type: String,
    pub logo: Option<String>,
    pub sign_in_count: i32,
    pub current_sign_in_at: Option<NaiveDateTime>,
    pub current_sign_in_ip: Option<String>,
    pub last_sign_in_at: Option<NaiveDateTime>,
    pub last_sign_in_ip: Option<String>,
    pub confirmed_at: Option<NaiveDateTime>,
    pub locked_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub const EMAIL: &'static str = "email";
    pub const GOOGLE: &'static str = "google";
    pub const GITHUB: &'static str = "github";
    pub const WECHAT: &'static str = "wechat";
    pub const FACEBOOK: &'static str = "facebook";

    pub fn status(&self) -> Option<String> {
        if let Some(_) = self.deleted_at {
            return Some(s!("nut.errors.user-delete"));
        }
        if let None = self.confirmed_at {
            return Some(s!("nut.errors.user-not-confirm"));
        }
        if let Some(_) = self.locked_at {
            return Some(s!("nut.errors.user-is-lock"));
        }
        None
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
    pub fn get_by_uid(db: &Db, uid: &String) -> Result<User> {
        let it = users::dsl::users
            .filter(users::dsl::uid.eq(uid))
            .first::<User>(db.deref())?;
        Ok(it)
    }
    pub fn get_by_email(db: &Db, email: &String) -> Result<User> {
        let it = users::dsl::users
            .filter(users::dsl::email.eq(email))
            .first::<User>(db.deref())?;
        Ok(it)
    }
    pub fn count(db: &Db) -> Result<i64> {
        let cnt: i64 = users::dsl::users.count().get_result(db.deref())?;
        Ok(cnt)
    }
    pub fn sign_in(
        db: &Db,
        user: &i32,
        remote: &SocketAddr,
        sign_in_count: i32,
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
    pub fn sign_up(db: &Db, name: &String, email: &String, password: &String) -> Result<User> {
        let now = Utc::now().naive_utc();
        let it: User = insert_into(users::dsl::users)
            .values((
                users::dsl::name.eq(name),
                users::dsl::email.eq(email),
                users::dsl::password.eq(&Some(hash::sum(password.as_bytes())?)),
                users::dsl::uid.eq(&Uuid::new_v4().to_string()),
                users::dsl::provider_type.eq(&s!(User::EMAIL)),
                users::dsl::provider_id.eq(email),
                users::dsl::logo.eq(&User::gravatar_logo(email)),
                users::dsl::updated_at.eq(&now),
            ))
            .get_result(db.deref())?;
        Ok(it)
    }
    pub fn set_confirmed(db: &Db, user: &i32) -> Result<()> {
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

    pub fn set_password(db: &Db, user: &i32, password: &String) -> Result<()> {
        let it = users::dsl::users.filter(users::dsl::id.eq(user));
        update(it)
            .set((
                users::dsl::password.eq(&Some(hash::sum(password.as_bytes())?)),
                users::dsl::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(db.deref())?;
        Ok(())
    }

    pub fn set_locked(db: &Db, user: &i32, ok: bool) -> Result<()> {
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
}

#[derive(Serialize, Queryable, Deserialize, Debug, Clone)]
pub struct Log {
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub message: String,
    pub ip: String,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
}

impl Log {
    pub fn list(db: &Db, user: &i32) -> Result<Vec<Log>> {
        let it = logs::dsl::logs
            .filter(logs::dsl::user_id.eq(user))
            .order(logs::dsl::created_at.desc())
            .get_results::<Log>(db.deref())?;
        Ok(it)
    }
    pub fn add<S: Serialize>(
        db: &Db,
        user: &i32,
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
