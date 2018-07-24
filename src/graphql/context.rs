use std::ops::Deref;
use std::sync::Arc;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use juniper;
use rocket::http::Status;

use super::super::{
    context::Context as AppContext,
    errors::Result,
    orm::{schema::users, PooledConnection as Db},
    plugins::nut::{
        dao::policy as policy_dao, graphql::users::mutation::ACT_SIGN_IN, models::Role,
    },
};

pub struct CurrentUser {
    pub id: i64,
    pub uid: String,
    pub email: String,
}

pub struct Context {
    pub home: String,
    pub locale: String,
    pub token: Option<String>,
    pub client_ip: String,
    pub app: Arc<AppContext>,
    pub db: Db,
}

impl juniper::Context for Context {}

impl Context {
    pub fn current_user(&self) -> Result<CurrentUser> {
        if let Some(ref token) = self.token {
            let payload = self.app.jwt.parse(token)?;
            if let Some(uid) = payload.get(super::UID.to_string()) {
                if let Some(act) = payload.get(super::ACT.to_string()) {
                    if act == ACT_SIGN_IN {
                        if let Some(uid) = uid.as_str() {
                            let uid = uid.to_string();
                            let db = self.db.deref();
                            let (id, email, confirmed_at, locked_at) = users::dsl::users
                                .select((
                                    users::dsl::id,
                                    users::dsl::email,
                                    users::dsl::confirmed_at,
                                    users::dsl::locked_at,
                                ))
                                .filter(users::dsl::uid.eq(&uid))
                                .first::<(i64, String, Option<NaiveDateTime>, Option<NaiveDateTime>)>(
                                    db,
                                )?;
                            // check is confirm
                            if None == confirmed_at {
                                return Err(
                                    t!(db, &self.locale, "nut.errors.user.not-confirmed").into()
                                );
                            }
                            // check is not lock
                            if let Some(_) = locked_at {
                                return Err(t!(db, &self.locale, "nut.errors.user.is-locked").into());
                            }
                            return Ok(CurrentUser {
                                id: id,
                                uid: uid,
                                email: email,
                            });
                        }
                    }
                }
            }
            return Err(Status::Unauthorized.reason.into());
        }
        Err(Status::NonAuthoritativeInformation.reason.into())
    }

    pub fn admin(&self) -> Result<CurrentUser> {
        let user = self.current_user()?;
        if policy_dao::is(self.db.deref(), &user.id, &Role::Admin) {
            return Ok(user);
        }
        Err(Status::Forbidden.reason.into())
    }

    pub fn client_ip(&self) -> Result<String> {
        Ok("ip".to_string())
    }
}
