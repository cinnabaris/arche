use std::ops::Deref;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use validator::Validate;

use super::super::super::super::{
    errors::Result,
    graphql::{context::Context, H},
    orm::schema::*,
    rfc::Utc as ToUtc,
};
use super::{
    models::{Locale, Log, Profile},
    mutation::{send_email, ACT_CONFIRM, ACT_RESET_PASSWORD, ACT_UNLOCK},
};

pub fn get_user_profile(ctx: &Context) -> Result<Profile> {
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

pub fn list_log(ctx: &Context) -> Result<Vec<Log>> {
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
pub struct ForgotUserPassword {
    #[validate(length(min = "2", max = "64"))]
    pub email: String,
}

impl ForgotUserPassword {
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
pub struct UnlockUser {
    #[validate(length(min = "2", max = "64"))]
    pub email: String,
}

impl UnlockUser {
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
pub struct ConfirmUser {
    #[validate(length(min = "2", max = "64"))]
    pub email: String,
}

impl ConfirmUser {
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

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
#[graphql(description = "Query locales by lang")]
pub struct ListLocaleByLang {
    #[validate(length(min = "2", max = "8"))]
    pub lang: String,
}

impl ListLocaleByLang {
    pub fn call(&self, ctx: &Context) -> Result<Vec<Locale>> {
        self.validate()?;
        let db = ctx.db.deref();
        let items = locales::dsl::locales
            .select((
                locales::dsl::id,
                locales::dsl::lang,
                locales::dsl::code,
                locales::dsl::message,
                locales::dsl::updated_at,
            ))
            .order(locales::dsl::code.asc())
            .filter(locales::dsl::lang.eq(&self.lang))
            .load::<(i64, String, String, String, NaiveDateTime)>(db)?;

        Ok(items
            .iter()
            .map(|(i, l, c, m, u)| Locale {
                id: i.to_string(),
                lang: l.clone(),
                code: c.clone(),
                message: m.clone(),
                updated_at: u.to_utc(),
            })
            .collect())
    }
}
