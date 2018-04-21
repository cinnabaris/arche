use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::net::SocketAddr;
use std::ops::Deref;
use std::time::Duration;

use diesel::Connection as DieselConnection;
use mime;
use rocket::http::RawStr;
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket_contrib::Json;
use serde_json::Value;
use validator::Validate;

use super::super::i18n::{Lang, Locale};
use super::super::jwt::{Home, Jwt};
use super::super::orm::Connection as Db;
use super::super::queue::{self, Queue};
use super::super::result::{Error, Result};
use super::models::{Log, User};
use super::workers::{Email, SEND_EMAIL};

#[get("/confirm/<token>")]
fn get_confirm_token(
    db: Db,
    remote: SocketAddr,
    jwt: State<Jwt>,
    token: &RawStr,
    lang: Lang,
) -> Flash<Redirect> {
    let call = || -> Result<String> {
        let Lang(lang) = lang;
        let token = try!(jwt.parse(&s!(token)));

        if let Some(act) = token["act"].as_str() {
            if act == ACT_CONFIRM {
                if let Some(email) = token["email"].as_str() {
                    if let Ok(_) = db.transaction::<_, Error, _>(|| {
                        let user = User::get_by_email(&db, &s!(email))?;
                        if let Some(_) = user.confirmed_at {
                            return Err(Locale::e(
                                &db,
                                &lang,
                                &s!("nut.errors.user-already-confirm"),
                                None::<Value>,
                            ));
                        }
                        User::confirm(&db, &user.id)?;
                        Log::add(
                            &db,
                            &user.id,
                            &lang,
                            &remote,
                            &s!("nut.logs.user.confirm"),
                            None::<Value>,
                        )?;
                        Ok(())
                    }) {
                        return Ok(Locale::t(&db, &lang, &s!("flash.success"), None::<Value>));
                    };
                }
            }
        }

        Err(Locale::e(
            &db,
            &lang,
            &s!("errors.bad-request"),
            None::<Value>,
        ))
    };

    match call() {
        Ok(msg) => Flash::success(Redirect::to("/"), &msg),
        Err(e) => Flash::error(Redirect::to("/"), e.description()),
    }
}

#[derive(Serialize, Deserialize, Debug, Validate)]
struct FmEmail {
    #[validate(email)]
    pub email: String,
}

#[post("/confirm", data = "<form>")]
fn post_confirm(
    db: Db,
    qu: State<Queue>,
    jwt: State<Jwt>,
    lang: Lang,
    home: Home,
    form: Json<FmEmail>,
) -> Result<Json<Value>> {
    let Lang(lang) = lang;
    let Home(home) = home;

    form.validate()?;
    let user = User::get_by_email(&db, &form.email)?;
    if let Some(_) = user.confirmed_at {
        return Err(Locale::e(
            &db,
            &lang,
            &s!("nut.errors.user-already-confirm"),
            None::<Value>,
        ));
    }

    send_email(
        &db,
        qu.deref(),
        jwt.deref(),
        &home,
        &lang,
        &form.email,
        ACT_CONFIRM,
    )?;
    Ok(Json(json!({})))
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct FmSignUp {
    #[validate(length(min = "2", max = "32"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "6"))]
    pub password: String,
}

#[post("/sign-up", data = "<form>")]
fn post_sign_up(
    db: Db,
    remote: SocketAddr,
    qu: State<Queue>,
    jwt: State<Jwt>,
    lang: Lang,
    home: Home,
    form: Json<FmSignUp>,
) -> Result<Json<Value>> {
    let Lang(lang) = lang;
    let Home(home) = home;

    form.validate()?;

    db.transaction::<_, Error, _>(|| {
        // check user not exist
        if User::is_email_exist(&db, &form.email)? {
            return Err(Locale::e(
                &db,
                &lang,
                &s!("nut.errors.email-already-exists"),
                None::<Value>,
            ));
        }
        // add email user
        let user = User::add_by_email(&db, &form.name, &form.email, &form.password)?;
        Log::add(
            &db,
            &user,
            &lang,
            &remote,
            &s!("nut.logs.user.sign-up"),
            None::<Value>,
        )?;
        Ok(())
    })?;

    send_email(
        &db,
        qu.deref(),
        jwt.deref(),
        &home,
        &lang,
        &form.email,
        ACT_CONFIRM,
    )?;

    Ok(Json(json!({})))
}

//-----------------------------------------------------------------------------
const ACT_CONFIRM: &'static str = "users.confirm";
// const ACT_UNLOCK: &'static str = "users.unlock";
// const ACT_RESET_PASSWORD: &'static str = "users.reset-password";

fn send_email(
    db: &Db,
    qu: &Queue,
    jwt: &Jwt,
    home: &String,
    lang: &String,
    email: &String,
    act: &'static str,
) -> Result<()> {
    let mut payload = json!({"email":email, "act":act});
    let token = try!(jwt.sum(&mut payload, Duration::from_secs(60 * 60 * 3)));
    let subject = Locale::t(
        db,
        lang,
        &format!("nut.{}.email-subject", act),
        None::<Value>,
    );
    let body = Locale::t(
        db,
        lang,
        &format!("nut.{}.email-body", act),
        Some(json!({"token": token, "home": home })),
    );
    queue::put(
        qu,
        &s!(SEND_EMAIL),
        &s!(mime::APPLICATION_JSON.as_ref()),
        6,
        &Email {
            to: email.clone(),
            subject: subject,
            body: body,
            attachments: BTreeMap::new(),
        },
    )?;
    Ok(())
}
