use std::ops::Deref;

use chrono::{Duration, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update, Connection};
use rocket::http::Status;
use validator::Validate;

use super::super::super::super::{
    errors::{Error, Result},
    graphql::{context::Context, ACT, H, UID},
    i18n,
    jwt::Jwt,
    orm::{schema::*, Connection as Db},
    queue, utils,
};
use super::super::{consumers, dao};
use super::models::SignIn;

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeUserPassword {
    pub current_password: String,
    #[validate(length(min = "6", max = "32"))]
    pub new_password: String,
}

impl ChangeUserPassword {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let db = ctx.db.deref();
        let user = ctx.current_user()?;
        let password = users::dsl::users
            .select(users::dsl::password)
            .filter(users::dsl::id.eq(&user.id))
            .first::<Option<Vec<u8>>>(db)?;
        if let Some(password) = password {
            if utils::hash::verify(&password, self.current_password.as_bytes()) {
                db.transaction::<_, Error, _>(|| {
                    dao::user::set_password(db, &user.id, &self.new_password)?;
                    l!(
                        db,
                        &user.id,
                        &ctx.client_ip,
                        &ctx.locale,
                        "nut.logs.user.change-password"
                    )?;
                    Ok(())
                })?;

                return Ok(H::new());
            }
        }
        Err(e!(db, &ctx.locale, "nut.errors.user.bad-password"))
    }
}
#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct UpdateUserProfile {
    #[validate(length(min = "1"))]
    pub logo: String,
    #[validate(length(min = "1"))]
    pub name: String,
}

impl UpdateUserProfile {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let db = ctx.db.deref();
        let user = ctx.current_user()?;
        let now = Utc::now().naive_utc();
        db.transaction::<_, Error, _>(|| {
            let it = users::dsl::users.filter(users::dsl::id.eq(&user.id));
            update(it)
                .set((
                    users::dsl::name.eq(&self.name),
                    users::dsl::logo.eq(&self.logo),
                    users::dsl::updated_at.eq(&now),
                ))
                .execute(db)?;
            l!(
                db,
                &user.id,
                &ctx.client_ip,
                &ctx.locale,
                "nut.logs.user.update-profile"
            )?;
            Ok(())
        })?;
        Ok(H::new())
    }
}

pub fn sign_out_user(ctx: &Context) -> Result<H> {
    let it = ctx.current_user()?;
    l!(
        ctx.db.deref(),
        &it.id,
        &ctx.client_ip,
        &ctx.locale,
        "nut.logs.user.sign-out"
    )?;
    Ok(H::new())
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct CreateLeaveWord {
    #[validate(length(min = "1"))]
    pub media_type: String,
    #[validate(length(min = "1"))]
    pub body: String,
}

impl CreateLeaveWord {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let db = ctx.db.deref();
        let now = Utc::now().naive_utc();
        insert_into(leave_words::dsl::leave_words)
            .values((
                leave_words::dsl::media_type.eq(&self.media_type),
                leave_words::dsl::body.eq(&self.body),
                leave_words::dsl::created_at.eq(&now),
            ))
            .execute(db)?;
        Ok(H::new())
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct SignInUserByEmail {
    #[validate(length(min = "1"))]
    pub email: String,
    #[validate(length(min = "6", max = "32"))]
    pub password: String,
}

impl SignInUserByEmail {
    pub fn call(&self, ctx: &Context) -> Result<SignIn> {
        self.validate()?;
        let db = ctx.db.deref();
        if let Ok((id, uid, password, confirmed_at, locked_at)) = users::dsl::users
            .select((
                users::dsl::id,
                users::dsl::uid,
                users::dsl::password,
                users::dsl::confirmed_at,
                users::dsl::locked_at,
            ))
            .filter(users::dsl::email.eq(&self.email))
            .first::<(
                i64,
                String,
                Option<Vec<u8>>,
                Option<NaiveDateTime>,
                Option<NaiveDateTime>,
            )>(db)
        {
            // check password
            if let Some(password) = password {
                if utils::hash::verify(&password, self.password.as_bytes()) {
                    // check is confirm
                    if None == confirmed_at {
                        return Err(t!(db, &ctx.locale, "nut.errors.user.not-confirmed").into());
                    }
                    // check is not lock
                    if let Some(_) = locked_at {
                        return Err(t!(db, &ctx.locale, "nut.errors.user.is-locked").into());
                    }
                    // set sign in
                    db.transaction::<_, Error, _>(|| {
                        dao::user::sign_in(db, &id, &ctx.client_ip)?;
                        l!(
                            db,
                            &id,
                            &ctx.client_ip,
                            &ctx.locale,
                            "nut.logs.user.sign-in"
                        )?;
                        Ok(())
                    })?;
                    // sum token
                    return Ok(SignIn {
                        token: ctx.app.jwt.sum(
                            &mut json!({
                                UID: uid,
                                ACT: ACT_SIGN_IN,
                                "groups": dao::policy::groups(db, &id)?,
                            }),
                            Duration::days(7),
                        )?,
                    });
                }
            }
        }
        Err(t!(db, &ctx.locale, "nut.errors.user.bad-password").into())
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct ResetUserPassword {
    #[validate(length(min = "1"))]
    pub token: String,
    #[validate(length(min = "6", max = "32"))]
    pub password: String,
}

impl ResetUserPassword {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let uid = parse_token(&ctx.app.jwt, &self.token, ACT_RESET_PASSWORD)?;
        let db = ctx.db.deref();

        db.transaction::<_, Error, _>(|| {
            let id = users::dsl::users
                .select(users::dsl::id)
                .filter(users::dsl::uid.eq(&uid))
                .first::<i64>(db)?;
            dao::user::set_password(db, &id, &self.password)?;
            l!(
                db,
                &id,
                &ctx.client_ip,
                &ctx.locale,
                "nut.logs.user.change-password"
            )?;
            Ok(())
        })?;
        Ok(H::new())
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct UnlockUser {
    #[validate(length(min = "1"))]
    pub token: String,
}

impl UnlockUser {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let uid = parse_token(&ctx.app.jwt, &self.token, ACT_UNLOCK)?;
        let db = ctx.db.deref();
        let (id, locked_at) = users::dsl::users
            .select((users::dsl::id, users::dsl::locked_at))
            .filter(users::dsl::uid.eq(&uid))
            .first::<(i64, Option<NaiveDateTime>)>(db)?;
        if let None = locked_at {
            return Err(t!(db, &ctx.locale, "nut.errors.user.not-locked").into());
        }
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((
                users::dsl::locked_at.eq(&None::<NaiveDateTime>),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(db)?;
        Ok(H::new())
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct ConfirmUser {
    #[validate(length(min = "1"))]
    pub token: String,
}

impl ConfirmUser {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let uid = parse_token(&ctx.app.jwt, &self.token, ACT_CONFIRM)?;
        let db = ctx.db.deref();
        let (id, confirmed_at) = users::dsl::users
            .select((users::dsl::id, users::dsl::confirmed_at))
            .filter(users::dsl::uid.eq(&uid))
            .first::<(i64, Option<NaiveDateTime>)>(db)?;
        if let Some(_) = confirmed_at {
            return Err(t!(db, &ctx.locale, "nut.errors.user.is-confirmed").into());
        }
        dao::user::confirm(db, &id)?;
        Ok(H::new())
    }
}

fn parse_token(jwt: &Jwt, token: &String, action: &'static str) -> Result<String> {
    let payload = jwt.parse(token)?;
    if let Some(uid) = payload.get(UID.to_string()) {
        if let Some(act) = payload.get(ACT.to_string()) {
            if act == action {
                if let Some(uid) = uid.as_str() {
                    return Ok(uid.to_string());
                }
            }
        }
    }
    return Err(Status::BadRequest.reason.into());
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Install {
    #[validate(length(min = "1", max = "32"))]
    pub name: String,
    #[validate(email, length(min = "2", max = "64"))]
    pub email: String,
    #[validate(length(min = "6", max = "32"))]
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
    #[validate(length(min = "6", max = "32"))]
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
pub const ACT_CONFIRM: &'static str = "user.confirm";
pub const ACT_UNLOCK: &'static str = "user.unlock";
pub const ACT_RESET_PASSWORD: &'static str = "user.reset-password";
pub fn send_email(
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
pub struct SignInUserByLine {
    #[validate(length(min = "2", max = "8"))]
    pub lang: String,
    #[validate(length(min = "1", max = "255"))]
    pub code: String,
    #[validate(length(min = "1"))]
    pub message: String,
}

impl SignInUserByLine {
    pub fn call(&self, _ctx: &Context) -> Result<String> {
        self.validate()?;
        // TODO
        Ok("".to_string())
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct RemoveLocale {
    #[validate(length(min = "1"))]
    pub id: String,
}

impl RemoveLocale {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let id: i64 = self.id.parse()?;
        ctx.admin()?;
        let db = ctx.db.deref();
        let it = locales::dsl::locales.filter(locales::dsl::id.eq(&id));
        delete(it).execute(db)?;
        Ok(H::new())
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
