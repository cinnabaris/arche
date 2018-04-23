use std::ops::Add;
use std::ops::Deref;

use base64;
use chrono::{Duration, NaiveDate, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::{delete, insert_into, update};

use super::super::orm::Connection as Db;
use super::super::result::Result;
use super::super::schema::{spree_role_users, spree_roles, spree_users};
use super::super::security::hash;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Role {
    pub id: i64,
    pub name: String,
}

impl Role {
    pub const ADMIN: &'static str = "admin";
    pub const ROOT: &'static str = "root";
    pub const MEMBER: &'static str = "member";

    pub fn find_or_create(db: &Db, name: &String) -> Result<i64> {
        let db = db.deref();
        match spree_roles::dsl::spree_roles
            .select(spree_roles::dsl::id)
            .filter(spree_roles::dsl::name.eq(name))
            .first::<i64>(db)
        {
            Ok(id) => Ok(id),
            Err(_) => {
                let it: Role = insert_into(spree_roles::dsl::spree_roles)
                    .values(spree_roles::dsl::name.eq(name))
                    .get_result(db)?;
                Ok(it.id)
            }
        }
    }

    pub fn users(db: &Db, role: &i64) -> Result<Vec<(i64, String)>> {
        let db = db.deref();
        let ids = spree_role_users::dsl::spree_role_users
            .select((
                spree_role_users::dsl::user_id,
                spree_role_users::dsl::nbf,
                spree_role_users::dsl::exp,
            ))
            .filter(spree_role_users::dsl::role_id.eq(role))
            .get_results::<(i64, NaiveDate, NaiveDate)>(db)?;

        let today = Utc::now().date().naive_utc();
        let mut items = Vec::new();
        for (id, nbf, exp) in ids {
            if nbf <= today && today <= exp {
                let it = spree_users::dsl::spree_users
                    .select((spree_users::dsl::id, spree_users::dsl::email))
                    .filter(spree_users::dsl::id.eq(&id))
                    .first::<(i64, String)>(db)?;
                items.push(it);
            }
        }
        Ok(items)
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub encrypted_password: String,
    pub password_salt: Option<String>,
    pub email: String,
    pub remember_token: Option<String>,
    pub persistence_token: Option<String>,
    pub reset_password_token: Option<String>,
    pub perishable_token: Option<String>,
    pub sign_in_count: i32,
    pub failed_attempts: i32,
    pub last_request_at: Option<NaiveDateTime>,
    pub current_sign_in_at: Option<NaiveDateTime>,
    pub last_sign_in_at: Option<NaiveDateTime>,
    pub current_sign_in_ip: Option<String>,
    pub last_sign_in_ip: Option<String>,
    pub login: Option<String>,
    pub ship_address_id: Option<i64>,
    pub bill_address_id: Option<i64>,
    pub authentication_token: Option<String>,
    pub unlock_token: Option<String>,
    pub locked_at: Option<NaiveDateTime>,
    pub reset_password_sent_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub spree_api_key: Option<String>,
    pub remember_created_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub confirmation_token: Option<String>,
    pub confirmed_at: Option<NaiveDateTime>,
    pub confirmation_sent_at: Option<NaiveDateTime>,
}

impl User {
    pub fn get_by_email(db: &Db, email: &String) -> Result<User> {
        let it = spree_users::dsl::spree_users
            .filter(spree_users::dsl::email.eq(email))
            .first::<User>(db.deref())?;
        Ok(it)
    }
    pub fn count(db: &Db) -> Result<i64> {
        let cnt: i64 = spree_users::dsl::spree_users
            .count()
            .get_result(db.deref())?;
        Ok(cnt)
    }
    pub fn sign_up(db: &Db, email: &String, password: &String) -> Result<User> {
        let now = Utc::now().naive_utc();
        let it: User = insert_into(spree_users::dsl::spree_users)
            .values((
                spree_users::dsl::email.eq(email),
                spree_users::dsl::encrypted_password
                    .eq(&base64::encode(&hash::sum(password.as_bytes())?)),
                spree_users::dsl::updated_at.eq(&now),
            ))
            .get_result(db.deref())?;
        Ok(it)
    }
    pub fn confirm(db: &Db, user: &i64) -> Result<()> {
        let it = spree_users::dsl::spree_users.filter(spree_users::dsl::id.eq(user));
        let now = Utc::now().naive_utc();
        update(it)
            .set((
                spree_users::dsl::confirmed_at.eq(&now),
                spree_users::dsl::updated_at.eq(&now),
            ))
            .execute(db.deref())?;
        Ok(())
    }
    pub fn lock(db: &Db, user: &i64, ok: bool) -> Result<()> {
        let it = spree_users::dsl::spree_users.filter(spree_users::dsl::id.eq(user));
        let now = Utc::now().naive_utc();
        update(it)
            .set((
                spree_users::dsl::locked_at.eq(if ok { Some(now) } else { None }),
                spree_users::dsl::updated_at.eq(&now),
            ))
            .execute(db.deref())?;
        Ok(())
    }

    pub fn roles(db: &Db, user: &i64) -> Result<Vec<String>> {
        let db = db.deref();
        let ids = spree_role_users::dsl::spree_role_users
            .select((
                spree_role_users::dsl::role_id,
                spree_role_users::dsl::nbf,
                spree_role_users::dsl::exp,
            ))
            .filter(spree_role_users::dsl::user_id.eq(user))
            .get_results::<(i64, NaiveDate, NaiveDate)>(db)?;

        let today = Utc::now().date().naive_utc();
        let mut items = Vec::new();
        for (id, nbf, exp) in ids {
            if nbf <= today && today <= exp {
                let n = spree_roles::dsl::spree_roles
                    .select(spree_roles::dsl::name)
                    .filter(spree_roles::dsl::id.eq(&id))
                    .first::<String>(db)?;
                items.push(n);
            }
        }
        Ok(items)
    }

    pub fn is(db: &Db, user: &i64, role: &i64) -> bool {
        if let Ok((nbf, exp)) = spree_role_users::dsl::spree_role_users
            .select((spree_role_users::dsl::nbf, spree_role_users::dsl::exp))
            .filter(spree_role_users::dsl::user_id.eq(user))
            .filter(spree_role_users::dsl::role_id.eq(role))
            .first::<(NaiveDate, NaiveDate)>(db.deref())
        {
            let today = Utc::now().date().naive_utc();
            return nbf <= today && today <= exp;
        }
        false
    }

    pub fn apply(db: &Db, user: &i64, role: &i64, days: i64) -> Result<()> {
        let db = db.deref();
        let nbf = Utc::now();
        let exp = nbf.add(Duration::days(days));
        let nbf = nbf.date().naive_utc();
        let exp = exp.date().naive_utc();

        let now = Utc::now().naive_utc();
        match spree_role_users::dsl::spree_role_users
            .select(spree_role_users::dsl::id)
            .filter(spree_role_users::dsl::user_id.eq(user))
            .filter(spree_role_users::dsl::role_id.eq(role))
            .first::<i64>(db)
        {
            Ok(id) => {
                let it = spree_role_users::dsl::spree_role_users
                    .filter(spree_role_users::dsl::id.eq(&id));
                update(it)
                    .set((
                        spree_role_users::dsl::nbf.eq(&nbf),
                        spree_role_users::dsl::exp.eq(&exp),
                        spree_role_users::dsl::updated_at.eq(&now),
                    ))
                    .execute(db)?;
                Ok(())
            }
            Err(_) => {
                insert_into(spree_role_users::dsl::spree_role_users)
                    .values((
                        spree_role_users::dsl::role_id.eq(role),
                        spree_role_users::dsl::user_id.eq(user),
                        spree_role_users::dsl::nbf.eq(&nbf),
                        spree_role_users::dsl::exp.eq(&exp),
                        spree_role_users::dsl::updated_at.eq(&now),
                    ))
                    .execute(db)?;
                Ok(())
            }
        }
    }

    pub fn deny(db: &Db, user: &i64, role: &i64) -> Result<usize> {
        let it = spree_role_users::dsl::spree_role_users
            .filter(spree_role_users::dsl::user_id.eq(user))
            .filter(spree_role_users::dsl::role_id.eq(role));

        let num = delete(it).execute(db.deref())?;
        Ok(num)
    }
}
