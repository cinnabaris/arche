use std::collections::BTreeMap;
use std::ops::Deref;

use rocket::http::RawStr;
use rocket::request::FlashMessage;
use rocket::State;
use rocket_contrib::{Json, Template};
use serde_json::Value;

use super::super::cache::{self, Cache};
use super::super::env;
use super::super::i18n::{Lang, Locale};
use super::super::orm::Connection as Db;
use super::super::result::Result;
use super::super::security::Encryptor;
use super::super::settings::Setting;

#[get("/")]
fn index(flash: Option<FlashMessage>) -> Template {
    let mut layout = json!({"lang":"en-US", "aaa":111, "bbb":"hi"});
    if let Some(flash) = flash {
        // let items = flash.map(|msg| (msg.name(), msg.msg())).collect();
        layout["flash"] = json!(flash.msg());
    }
    Template::render("index", layout)
}

#[get("/locales/<lang>")]
fn get_locales(ch: State<Cache>, db: Db, lang: &RawStr) -> Result<Json<BTreeMap<String, String>>> {
    let items = cache::get(ch.deref(), &format!("locales/{}", lang), 1, || {
        let items = Locale::map_by_lang(&db, &s!(lang))?;
        Ok(items)
    })?;

    Ok(Json(items))
}

#[get("/layout")]
fn get_layout(
    db: Db,
    l: Lang,
    cfg: State<env::Config>,
    sec: State<Encryptor>,
) -> Result<Json<Value>> {
    let sec = sec.deref();
    let Lang(l) = l;

    // site info
    let mut site = BTreeMap::new();

    for it in vec!["title", "subhead", "keywords", "description", "copyright"] {
        let code = String::from("site.") + it;
        site.entry(it)
            .or_insert(json!(Locale::t(&db, &l, &code, None::<Value>)));
    }
    // author
    site.insert(
        "author",
        json!(match Setting::get(&db, sec, &s!("site.author")) {
            Ok(v) => v,
            Err(_) => {
                let mut it = BTreeMap::new();
                it.insert(s!("name"), s!("who-am-i"));
                it.insert(s!("email"), s!("master@change-me.com"));
                it
            }
        }),
    );
    // favicon
    site.insert(
        "favicon",
        json!(match Setting::get(&db, sec, &s!("site.favicon")) {
            Ok(v) => v,
            Err(_) => s!("/favicon.png"),
        }),
    );

    // i18n
    site.insert("locale", json!(l));
    site.insert("languages", json!(cfg.languages));

    Ok(Json(json!(site)))
}

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
use serde_json::{self, Value};

use super::super::i18n::Locale;
use super::super::jwt::{Home, Jwt};
use super::super::orm::Connection as Db;
use super::super::queue::{self, Queue};
use super::super::result::{Error, Result};
use super::super::spree::guards::{CurrentUser, Session};
use super::super::spree::models::User;
use super::models::Log;
use super::workers::{Email, SEND_EMAIL};

#[delete("/sign-out")]
fn delete_sign_out(db: Db, lang: Lang, remote: SocketAddr, user: CurrentUser) -> Result<Json<()>> {
    let Lang(lang) = lang;
    Log::add(
        &db,
        &user.id,
        &s!(lang),
        &remote,
        &s!("nut.logs.user.sign-out"),
        None::<Value>,
    )?;
    Ok(Json(()))
}

#[get("/logs")]
fn get_logs(db: Db, user: CurrentUser) -> Result<Json<Vec<Log>>> {
    let items = Log::list(&db, &user.id)?;
    Ok(Json(items))
}

//-----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Validate)]
struct FmSignIn {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "6"))]
    pub password: String,
}

#[post("/sign-in", data = "<form>")]
fn post_sign_in(
    db: Db,
    remote: SocketAddr,
    jwt: State<Jwt>,
    lang: Lang,
    form: Json<FmSignIn>,
) -> Result<Json<Value>> {
    let Lang(lang) = lang;

    form.validate()?;

    let it = User::get_by_email(&db, &form.email)?;

    if User::chk_password(&it.encrypted_password, &form.password) {
        if let Some(code) = it.status() {
            return Err(Locale::e(&db, &lang, &code, None::<Value>));
        }
        let token = db.transaction::<_, Error, _>(|| {
            User::sign_in(
                &db,
                &it.id,
                &remote,
                it.sign_in_count,
                it.current_sign_in_ip,
                it.current_sign_in_at,
            )?;
            Log::add(
                &db,
                &it.id,
                &lang,
                &remote,
                &s!("nut.logs.user.sign-in.success"),
                None::<Value>,
            )?;

            let token = jwt.sum(
                &mut serde_json::to_value(&Session {
                    uid: it.login,
                    roles: User::roles(&db, &it.id)?,
                })?,
                Duration::from_secs(60 * 60 * 24),
            )?;
            Ok(token)
        })?;

        return Ok(Json(json!({ "token": token })));
    }
    Log::add(
        &db,
        &it.id,
        &lang,
        &remote,
        &s!("nut.logs.user.sign-in.failed"),
        None::<Value>,
    )?;

    Err(Locale::e(
        &db,
        &lang,
        &s!("nut.errors.user-bad-password"),
        None::<Value>,
    ))
}

#[derive(Serialize, Deserialize, Debug, Validate)]
struct FmResetPassword {
    #[validate(length(min = "6"))]
    pub password: String,
    #[validate(length(min = "1"))]
    pub token: String,
}

#[post("/reset-password", data = "<form>")]
fn post_reset_password(
    db: Db,
    jwt: State<Jwt>,
    lang: Lang,
    remote: SocketAddr,
    form: Json<FmResetPassword>,
) -> Result<Json<Value>> {
    let Lang(lang) = lang;
    let token = jwt.parse(&form.token)?;

    if let Some(act) = token["act"].as_str() {
        if act == ACT_RESET_PASSWORD {
            if let Some(email) = token["email"].as_str() {
                if let Ok(_) = db.transaction::<_, Error, _>(|| {
                    let user = User::get_by_email(&db, &s!(email))?;
                    User::password(&db, &user.id, &form.password)?;
                    Log::add(
                        &db,
                        &user.id,
                        &lang,
                        &remote,
                        &s!("nut.logs.user.reset-password"),
                        None::<Value>,
                    )?;
                    Ok(())
                }) {
                    return Ok(Json(json!({})));
                }
            }
        }
    }

    Err(Locale::e(
        &db,
        &lang,
        &s!("errors.bad-request"),
        None::<Value>,
    ))
}

#[post("/forgot-password", data = "<form>")]
fn post_forgot_password(
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
    User::get_by_email(&db, &form.email)?;

    send_email(
        &db,
        qu.deref(),
        jwt.deref(),
        &home,
        &lang,
        &form.email,
        ACT_RESET_PASSWORD,
    )?;
    Ok(Json(json!({})))
}

#[get("/unlock/<token>")]
fn get_unlock_token(
    db: Db,
    remote: SocketAddr,
    jwt: State<Jwt>,
    token: &RawStr,
    lang: Lang,
) -> Flash<Redirect> {
    let call = || -> Result<String> {
        let Lang(lang) = lang;
        let token = jwt.parse(&s!(token))?;

        if let Some(act) = token["act"].as_str() {
            if act == ACT_UNLOCK {
                if let Some(email) = token["email"].as_str() {
                    if let Ok(_) = db.transaction::<_, Error, _>(|| {
                        let user = User::get_by_email(&db, &s!(email))?;
                        if let None = user.locked_at {
                            return Err(Locale::e(
                                &db,
                                &lang,
                                &s!("nut.errors.user-not-lock"),
                                None::<Value>,
                            ));
                        }
                        User::lock(&db, &user.id, false)?;
                        Log::add(
                            &db,
                            &user.id,
                            &lang,
                            &remote,
                            &s!("nut.logs.user.unlock"),
                            None::<Value>,
                        )?;
                        Ok(())
                    }) {
                        return Ok(Locale::t(&db, &lang, &s!("flash.success"), None::<Value>));
                    }
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

#[post("/unlock", data = "<form>")]
fn post_unlock(
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
    if let None = user.locked_at {
        return Err(Locale::e(
            &db,
            &lang,
            &s!("nut.errors.user-not-lock"),
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
        ACT_UNLOCK,
    )?;
    Ok(Json(json!({})))
}

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
        let token = jwt.parse(&s!(token))?;

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
                    }
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
        if let Ok(_) = User::get_by_email(&db, &form.email) {
            return Err(Locale::e(
                &db,
                &lang,
                &s!("nut.errors.email-already-exists"),
                None::<Value>,
            ));
        }
        // add email user
        let user = User::sign_up(&db, &form.email, &form.password)?;
        Log::add(
            &db,
            &user.id,
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
const ACT_UNLOCK: &'static str = "users.unlock";
const ACT_RESET_PASSWORD: &'static str = "users.reset-password";

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
    let token = jwt.sum(&mut payload, Duration::from_secs(60 * 60 * 3))?;
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
