use std::ops::Deref;

use chrono::{Duration, NaiveDate, NaiveDateTime, Utc};
use diesel::{prelude::*, update, Connection};
use rocket::http::Status;
use serde_json;
use validator::Validate;

use super::super::super::super::super::{
    errors::{Error, Result},
    graphql::{context::Context, ACT, H, UID},
    i18n,
    jwt::Jwt,
    orm::{schema::*, Connection as Db},
    queue, utils,
};
use super::super::super::{
    consumers,
    dao::{self, role::Type as RoleType},
};
use super::models::SignIn;

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Policy {
    #[validate(length(min = "1"))]
    pub name: String,
    pub enable: bool,
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePolicy {
    #[validate(length(min = "1"))]
    pub user: String,
    #[validate(length(min = "1"))]
    pub nbf: String,
    #[validate(length(min = "1"))]
    pub exp: String,
    pub policies: String,
}

impl UpdatePolicy {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        ctx.admin()?;
        let db = ctx.db.deref();
        let user: i64 = self.user.parse()?;
        if dao::policy::is(db, &user, &RoleType::Root) {
            return Err(Status::Forbidden.reason.into());
        }

        let policies: Vec<Policy> = serde_json::from_str(&self.policies)?;
        for it in policies.iter() {
            it.validate()?;
        }

        db.transaction::<_, Error, _>(|| {
            for it in policies.iter() {
                let role = RoleType::By(it.name.clone());

                if it.enable {
                    let nbf = NaiveDate::parse_from_str(&self.nbf, utils::DATE_FORMAT)?;
                    let exp = NaiveDate::parse_from_str(&self.exp, utils::DATE_FORMAT)?;
                    dao::policy::apply_by_range(db, &user, &role, &None, &None, &nbf, &exp)?;
                    l!(
                        db,
                        &user,
                        &ctx.client_ip,
                        &ctx.locale,
                        "nut.logs.role.apply",
                        &Some(json!({
                            "name":format!("{}", role),
                            "type": None::<String>,
                            "id": None::<i64>,
                            "ttl": format!("{}-{}", nbf, exp)
                        }))
                    )?;
                } else {
                    dao::policy::deny(db, &user, &role, &None, &None)?;
                    l!(
                        db,
                        &user,
                        &ctx.client_ip,
                        &ctx.locale,
                        "nut.logs.role.deny",
                        &Some(json!({
                            "name":format!("{}", role),
                            "type": None::<String>,
                            "id": None::<i64>
                        }))
                    )?;
                }
            }
            Ok(())
        })?;

        Ok(H::new())
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePassword {
    pub current_password: String,
    #[validate(length(min = "6", max = "32"))]
    pub new_password: String,
}

impl ChangePassword {
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
pub struct UpdateProfile {
    #[validate(length(min = "1"))]
    pub logo: String,
    #[validate(length(min = "1"))]
    pub name: String,
}

impl UpdateProfile {
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

pub fn sign_out(ctx: &Context) -> Result<H> {
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
pub struct SignInByEmail {
    #[validate(length(min = "1"))]
    pub email: String,
    #[validate(length(min = "6", max = "32"))]
    pub password: String,
}

impl SignInByEmail {
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
pub struct ResetPassword {
    #[validate(length(min = "1"))]
    pub token: String,
    #[validate(length(min = "6", max = "32"))]
    pub password: String,
}

impl ResetPassword {
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
pub struct Lock {
    #[validate(length(min = "1"))]
    pub id: String,
}

impl Lock {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        ctx.admin()?;
        let db = ctx.db.deref();
        let now = Utc::now().naive_utc();
        let id = self.id.parse::<i64>()?;
        db.transaction::<_, Error, _>(|| {
            let it = users::dsl::users.filter(users::dsl::id.eq(&id));
            update(it)
                .set((
                    users::dsl::locked_at.eq(&now),
                    users::dsl::updated_at.eq(&now),
                ))
                .execute(db)?;
            l!(db, &id, &ctx.client_ip, &ctx.locale, "nut.logs.user.lock")?;
            Ok(())
        })?;
        Ok(H::new())
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Unlock {
    #[validate(length(min = "1"))]
    pub token: String,
}

impl Unlock {
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
        db.transaction::<_, Error, _>(|| {
            let it = users::dsl::users.filter(users::dsl::id.eq(id));
            update(it)
                .set((
                    users::dsl::locked_at.eq(&None::<NaiveDateTime>),
                    users::dsl::updated_at.eq(&now),
                ))
                .execute(db)?;
            l!(db, &id, &ctx.client_ip, &ctx.locale, "nut.logs.user.unlock")?;
            Ok(())
        })?;
        Ok(H::new())
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Confirm {
    #[validate(length(min = "1"))]
    pub token: String,
}

impl Confirm {
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
        db.transaction::<_, Error, _>(|| {
            dao::user::confirm(db, &id)?;
            l!(
                db,
                &id,
                &ctx.client_ip,
                &ctx.locale,
                "nut.logs.user.confirm"
            )?;
            Ok(())
        })?;
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
pub struct SignUp {
    #[validate(length(min = "1", max = "32"))]
    pub name: String,
    #[validate(email, length(min = "2", max = "64"))]
    pub email: String,
    #[validate(length(min = "6", max = "32"))]
    pub password: String,
}

impl SignUp {
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
