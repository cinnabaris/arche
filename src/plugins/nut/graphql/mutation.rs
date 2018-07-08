use std::ops::Deref;

use chrono::{Duration, NaiveDateTime};
use diesel::prelude::*;
use diesel::Connection;
use validator::Validate;

use super::super::super::super::{
    errors::{Error, Result},
    graphql::{context::Context, ACT, H, UID},
    i18n,
    jwt::Jwt,
    orm::{schema::users, Connection as Db},
    queue,
};
use super::super::{consumers, dao};

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
pub struct Install {
    #[validate(length(min = "1", max = "32"))]
    pub name: String,
    #[validate(email, length(min = "2", max = "64"))]
    pub email: String,
    #[validate(length(min = "1"))]
    pub password: String,
}

impl Install {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let db = ctx.db.deref();
        db.transaction::<_, Error, _>(|| {
            if dao::user::count(db)? > 0 {
                return Err(t!(db, &ctx.locale, "nut.errors.database-not-empty").into());
            }
            let (user, _) = dao::user::add_by_email(db, &self.name, &self.email, &self.password)?;
            l!(
                db,
                &user,
                &ctx.client_ip,
                &ctx.locale,
                "nut.logs.user.sign-up"
            )?;
            dao::user::confirm(db, &user)?;
            l!(
                db,
                &user,
                &ctx.client_ip,
                &ctx.locale,
                "nut.logs.user.confirm"
            )?;
            for it in vec![dao::role::Type::Admin, dao::role::Type::Root] {
                let ttl = Duration::weeks(1 << 12);
                dao::policy::apply(db, &user, &it, &None, &None, ttl)?;
                l!(
                    db,
                    &user,
                    &ctx.client_ip,
                    &ctx.locale,
                    "nut.logs.role.apply",
                    &Some(json!({
                            "name":format!("{}", it),
                            "type": None::<String>,
                            "id": None::<i64>,
                            "ttl": format!("{}", ttl)
                        }))
                )?;
            }
            Ok(H::new())
        })
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct SignUpUser {
    #[validate(length(min = "1", max = "32"))]
    pub name: String,
    #[validate(email, length(min = "2", max = "64"))]
    pub email: String,
    #[validate(length(min = "1"))]
    pub password: String,
}

impl SignUpUser {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let db = ctx.db.deref();
        let (_id, uid) = db.transaction::<_, Error, _>(|| {
            if dao::user::is_email_exist(db, &self.email)? {
                return Err(t!(db, &ctx.locale, "nut.errors.user.email-already-exist").into());
            }
            let (id, uid) = dao::user::add_by_email(db, &self.name, &self.email, &self.password)?;
            l!(
                db,
                &id,
                &ctx.client_ip,
                &ctx.locale,
                "nut.logs.user.sign-up"
            )?;
            Ok((id, uid))
        })?;
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
pub const ACT_SIGN_IN: &'static str = "user.sign-in";
const ACT_CONFIRM: &'static str = "user.confirm";
const ACT_UNLOCK: &'static str = "user.unlock";
const ACT_RESET_PASSWORD: &'static str = "user.reset-password";
fn send_email(
    db: &Db,
    home: &String,
    jwt: &Jwt,
    producer: &queue::Producer,
    act: &'static str,
    lang: &String,
    email: &String,
    uid: &String,
) -> Result<()> {
    let pre = "nut.emails";
    let token = jwt.sum(
        &mut json!({
        UID: uid,
        ACT: act,
    }),
        Duration::days(1),
    )?;
    let args = Some(json!({
        "home": home,
        "token": token,
    }));
    queue::push(
        producer,
        consumers::send_mail::NAME,
        1,
        &consumers::send_mail::Mail {
            to: email.clone(),
            subject: i18n::tr(db, lang, &format!("{}.{}.subject", pre, act), &args)?,
            body: i18n::tr(db, lang, &format!("{}.{}.body", pre, act), &args)?,
            attachments: None,
        },
    )?;
    Ok(())
}

// https://developers.line.me/en/docs/line-login/web/integrate-line-login/
#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct SignInByLine {
    #[validate(length(min = "2", max = "8"))]
    pub lang: String,
    #[validate(length(min = "1", max = "255"))]
    pub code: String,
    #[validate(length(min = "1"))]
    pub message: String,
}

impl SignInByLine {
    pub fn call(&self, _ctx: &Context) -> Result<String> {
        self.validate()?;
        // TODO
        Ok("".to_string())
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct UpdateLocale {
    #[validate(length(min = "1", max = "255"))]
    pub code: String,
    #[validate(length(min = "1"))]
    pub message: String,
}

impl UpdateLocale {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        ctx.admin()?;
        let db = ctx.db.deref();
        i18n::set(db, &ctx.locale, &self.code, &self.message)?;
        Ok(H::new())
    }
}
