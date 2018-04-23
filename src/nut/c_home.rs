// use std::collections::BTreeMap;
// use std::net::SocketAddr;
// use std::ops::Deref;
//
// use diesel::Connection as DieselConnection;
// use rocket::http::RawStr;
// use rocket::request::FlashMessage;
// use rocket::State;
// use rocket_contrib::{Json, Template};
// use serde_json::Value;
// use validator::Validate;
//
// use super::super::cache::{self, Cache};
// use super::super::env;
// use super::super::i18n::{Lang, Locale};
// use super::super::orm::Connection as Db;
// use super::super::result::{Error, Result};
// use super::super::security::Encryptor;
// use super::super::settings::Setting;
// use super::c_users::FmSignUp;
// use super::models::{Log, Policy, Role, User, ROLE_ADMIN, ROLE_ROOT};
//
// #[get("/")]
// fn index(flash: Option<FlashMessage>) -> Template {
//     let mut layout = json!({"lang":"en-US", "aaa":111, "bbb":"hi"});
//     if let Some(flash) = flash {
//         // let items = flash.map(|msg| (msg.name(), msg.msg())).collect();
//         layout["flash"] = json!(flash.msg());
//     }
//     Template::render("index", layout)
// }
//
// #[get("/locales/<lang>")]
// fn get_locales(ch: State<Cache>, db: Db, lang: &RawStr) -> Result<Json<BTreeMap<String, String>>> {
//     let items = cache::get(ch.deref(), &format!("locales/{}", lang), 1, || {
//         let items = Locale::map_by_lang(&db, &s!(lang))?;
//         Ok(items)
//     })?;
//
//     Ok(Json(items))
// }
//
// #[get("/layout")]
// fn get_layout(
//     db: Db,
//     l: Lang,
//     cfg: State<env::Config>,
//     sec: State<Encryptor>,
// ) -> Result<Json<Value>> {
//     let sec = sec.deref();
//     let Lang(l) = l;
//
//     // site info
//     let mut site = BTreeMap::new();
//
//     for it in vec!["title", "subhead", "keywords", "description", "copyright"] {
//         let code = String::from("site.") + it;
//         site.entry(it)
//             .or_insert(json!(Locale::t(&db, &l, &code, None::<Value>)));
//     }
//     // author
//     site.insert(
//         "author",
//         json!(match Setting::get(&db, sec, &s!("site.author")) {
//             Ok(v) => v,
//             Err(_) => {
//                 let mut it = BTreeMap::new();
//                 it.insert(s!("name"), s!("who-am-i"));
//                 it.insert(s!("email"), s!("master@change-me.com"));
//                 it
//             }
//         }),
//     );
//     // favicon
//     site.insert(
//         "favicon",
//         json!(match Setting::get(&db, sec, &s!("site.favicon")) {
//             Ok(v) => v,
//             Err(_) => s!("/favicon.png"),
//         }),
//     );
//
//     // i18n
//     site.insert("locale", json!(l));
//     site.insert("languages", json!(cfg.languages));
//
//     Ok(Json(json!(site)))
// }
//
// #[post("/install", data = "<form>")]
// fn post_install(db: Db, remote: SocketAddr, l: Lang, form: Json<FmSignUp>) -> Result<Json<Value>> {
//     let Lang(l) = l;
//     form.validate()?;
//
//     db.transaction::<_, Error, _>(|| {
//         // check database is empty
//         let count = User::count(&db)?;
//         if count > 0 {
//             return Err(Locale::e(
//                 &db,
//                 &l,
//                 &s!("nut.errors.database-not-empty"),
//                 None::<Value>,
//             ));
//         }
//         // add email user
//         let user = User::add_by_email(&db, &form.name, &form.email, &form.password)?;
//         Log::add(
//             &db,
//             &user,
//             &l,
//             &remote,
//             &s!("nut.logs.user.sign-up"),
//             None::<Value>,
//         )?;
//         // confirm user
//         User::confirm(&db, &user)?;
//         Log::add(
//             &db,
//             &user,
//             &l,
//             &remote,
//             &s!("nut.logs.user.confirm"),
//             None::<Value>,
//         )?;
//         // add roles
//         for n in vec![ROLE_ROOT, ROLE_ADMIN] {
//             let role = Role::get(&db, &s!(n), &None, &None)?;
//             Policy::apply(&db, &user, &role, 365 * 120)?;
//             Log::add(
//                 &db,
//                 &user,
//                 &l,
//                 &remote,
//                 &s!("nut.logs.user.apply"),
//                 Some(json!({ "name": n })),
//             )?;
//         }
//         Ok(())
//     })?;
//
//     Ok(Json(json!({})))
// }
