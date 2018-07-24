use std::ops::Deref;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use validator::Validate;

use super::super::super::super::super::{
    errors::Result,
    graphql::{context::Context, H},
    orm::schema::*,
    rfc::UtcDateTime,
};
use super::super::super::models;
use super::{
    models::{Log, Policy, Profile, User},
    mutation::{send_email, ACT_CONFIRM, ACT_RESET_PASSWORD, ACT_UNLOCK},
};

// resource_id is null
pub fn policies(ctx: &Context) -> Result<Vec<Policy>> {
    let user = ctx.current_user()?;
    let mut items = Vec::new();
    let db = ctx.db.deref();
    for it in policies::dsl::policies
        .filter(policies::dsl::user_id.eq(&user.id))
        .load::<models::Policy>(db)?
    {
        if it.enable() {
            items.push(it.into());
        }
    }
    Ok(items)
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPolicyByUser {
    #[validate(length(min = "1"))]
    pub id: String,
}

impl ListPolicyByUser {
    pub fn call(&self, ctx: &Context) -> Result<Vec<Policy>> {
        self.validate()?;
        ctx.admin()?;
        let db = ctx.db.deref();
        let user: i64 = self.id.parse()?;

        let mut items = Vec::new();
        for it in policies::dsl::policies
            .filter(policies::dsl::user_id.eq(&user))
            .load::<models::Policy>(db)?
        {
            if it.enable() {
                items.push(it.into());
            }
        }
        Ok(items)
    }
}

pub fn list(ctx: &Context) -> Result<Vec<User>> {
    ctx.admin()?;
    let db = ctx.db.deref();
    let items = users::dsl::users
        .select((
            users::dsl::id,
            users::dsl::name,
            users::dsl::email,
            users::dsl::sign_in_count,
            users::dsl::last_sign_in_at,
            users::dsl::last_sign_in_ip,
            users::dsl::current_sign_in_at,
            users::dsl::current_sign_in_ip,
        ))
        .order(users::dsl::updated_at.desc())
        .load::<(
            i64,
            String,
            String,
            i64,
            Option<NaiveDateTime>,
            Option<String>,
            Option<NaiveDateTime>,
            Option<String>,
        )>(db)?;

    Ok(items
        .iter()
        .map(
            |(
                id,
                name,
                email,
                sign_in_count,
                last_sign_in_at,
                last_sign_in_ip,
                current_sign_in_at,
                current_sign_in_ip,
            )| User {
                id: id.to_string(),
                name: name.clone(),
                email: email.clone(),
                sign_in_count: sign_in_count.to_string(),
                last_sign_in_at: match last_sign_in_at {
                    Some(d) => Some(d.to_utc()),
                    None => None,
                },
                last_sign_in_ip: last_sign_in_ip.clone(),
                current_sign_in_at: match current_sign_in_at {
                    Some(d) => Some(d.to_utc()),
                    None => None,
                },
                current_sign_in_ip: current_sign_in_ip.clone(),
            },
        )
        .collect())
}

pub fn profile(ctx: &Context) -> Result<Profile> {
    let user = ctx.current_user()?;
    let db = ctx.db.deref();
    let (name, email, logo) = users::dsl::users
        .select((users::dsl::name, users::dsl::email, users::dsl::logo))
        .filter(users::dsl::id.eq(&user.id))
        .first::<(String, String, String)>(db)?;
    Ok(Profile {
        name: name,
        email: email,
        logo: logo,
    })
}

pub fn logs(ctx: &Context) -> Result<Vec<Log>> {
    let user = ctx.current_user()?;
    let db = ctx.db.deref();
    let items = logs::dsl::logs
        .select((
            logs::dsl::id,
            logs::dsl::message,
            logs::dsl::ip,
            logs::dsl::created_at,
        ))
        .order(logs::dsl::created_at.desc())
        .filter(logs::dsl::user_id.eq(&user.id))
        .load::<(i64, String, String, NaiveDateTime)>(db)?;

    Ok(items
        .iter()
        .map(|(id, msg, ip, ts)| Log {
            id: id.to_string(),
            message: msg.clone(),
            ip: ip.clone(),
            created_at: ts.to_utc(),
        })
        .collect())
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct ForgotPassword {
    #[validate(length(min = "2", max = "64"))]
    pub email: String,
}

impl ForgotPassword {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let db = ctx.db.deref();
        let uid = users::dsl::users
            .select(users::dsl::uid)
            .filter(users::dsl::email.eq(&self.email))
            .first::<String>(db)?;
        send_email(
            db,
            &ctx.home,
            &ctx.app.jwt,
            &ctx.app.producer,
            ACT_RESET_PASSWORD,
            &ctx.locale,
            &self.email,
            &uid,
        )?;
        Ok(H::new())
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Unlock {
    #[validate(length(min = "2", max = "64"))]
    pub email: String,
}

impl Unlock {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let db = ctx.db.deref();
        let (uid, locked_at) = users::dsl::users
            .select((users::dsl::uid, users::dsl::locked_at))
            .filter(users::dsl::email.eq(&self.email))
            .first::<(String, Option<NaiveDateTime>)>(db)?;
        // check is lock
        if let None = locked_at {
            return Err(t!(db, &ctx.locale, "nut.errors.user.not-locked").into());
        }
        send_email(
            db,
            &ctx.home,
            &ctx.app.jwt,
            &ctx.app.producer,
            ACT_UNLOCK,
            &ctx.locale,
            &self.email,
            &uid,
        )?;
        Ok(H::new())
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Confirm {
    #[validate(length(min = "2", max = "64"))]
    pub email: String,
}

impl Confirm {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let db = ctx.db.deref();
        let (uid, confirmed_at) = users::dsl::users
            .select((users::dsl::uid, users::dsl::confirmed_at))
            .filter(users::dsl::email.eq(&self.email))
            .first::<(String, Option<NaiveDateTime>)>(db)?;
        // check is not confirm
        if let Some(_) = confirmed_at {
            return Err(t!(db, &ctx.locale, "nut.errors.user.is-confirmed").into());
        }
        send_email(
            db,
            &ctx.home,
            &ctx.app.jwt,
            &ctx.app.producer,
            ACT_CONFIRM,
            &ctx.locale,
            &self.email,
            &uid,
        )?;
        Ok(H::new())
    }
}
