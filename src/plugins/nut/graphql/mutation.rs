use std::ops::Deref;

use chrono::Duration;
use diesel::Connection;
use validator::Validate;

use super::super::super::super::{
    errors::{Error, Result},
    graphql::{context::Context, H},
    i18n,
    jwt::Jwt,
    orm::Connection as Db,
    queue,
};
use super::super::{consumers, dao};

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Install {
    #[validate(length(min = "1", max = "32"))]
    pub name: String,
    #[validate(length(min = "2", max = "64"))]
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
                return Err(t!(db, &ctx.locale, "nut.errors.user.database-not-empty").into());
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
            for it in vec![dao::role::Role::Admin, dao::role::Role::Root] {
                let ttl = Duration::weeks(1 << 12);
                dao::policy::apply(db, &user, &it, &None, &None, ttl)?;
                l!(
                    db,
                    &user,
                    &ctx.client_ip,
                    &ctx.locale,
                    "nut.logs.role.apply",
                    &Some(json!({
                            "name":it,
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
    #[validate(length(min = "2", max = "64"))]
    pub email: String,
    #[validate(length(min = "1"))]
    pub password: String,
}

impl SignUpUser {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let db = ctx.db.deref();
        db.transaction::<_, Error, _>(|| {
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
        })
    }
}
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
        "uid": uid,
        "act": act,
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
    #[validate(length(min = "2", max = "8"))]
    pub lang: String,
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
        i18n::set(db, &self.lang, &self.code, &self.message)?;
        Ok(H::new())
    }
}
