use std::collections::BTreeMap;
use std::ops::Deref;

use rocket::http::RawStr;
use rocket::State;
use rocket_contrib::Json;

use super::super::cache::{self, Cache};
use super::super::i18n::Locale;
use super::super::orm::Connection as Db;
use super::super::result::Result;

#[get("/locales/<lang>")]
fn get_locales(ch: State<Cache>, db: Db, lang: &RawStr) -> Result<Json<BTreeMap<String, String>>> {
    let items = cache::get(ch.deref(), &format!("locales/{}", lang), 1, || {
        let items = Locale::map_by_lang(&db, &s!(lang))?;
        Ok(items)
    })?;

    Ok(Json(items))
}

// #[get("/layout")]
// fn get_layout(
//     db: Connection,
//     l: Lang,
//     cfg: State<env::Config>,
//     sec: State<Sodium>,
// ) -> Result<Json<Value>> {
//     let db = db.deref();
//     let sec = sec.deref();
//     let Lang(l) = l;
//
//     // site info
//     let mut site = BTreeMap::new();
//     for it in vec!["title", "subhead", "keywords", "description", "copyright"] {
//         let code = String::from("site.") + it;
//         site.entry(it)
//             .or_insert(json!(t!(db, &l, &code, None::<Value>)));
//     }
//     // author
//     site.insert(
//         "author",
//         json!(match Setting::get(db, sec, s!("site.author")) {
//             Ok(v) => {
//                 let it: BTreeMap<String, String> = try!(serde_json::from_slice(&v));
//                 it
//             }
//             Err(_) => {
//                 let mut it = BTreeMap::new();
//                 it.insert("name".to_string(), "who-am-i".to_string());
//                 it.insert("email".to_string(), "master@change-me.com".to_string());
//                 it
//             }
//         }),
//     );
//     // favicon
//     site.insert(
//         "favicon",
//         json!(match Setting::get(db, sec, s!("site.favicon")) {
//             Ok(v) => try!(String::from_utf8(v)),
//             Err(_) => "/favicon.png".to_string(),
//         }),
//     );
//
//     // i18n
//     site.insert("locale", json!(l));
//     site.insert("languages", json!(cfg.languages));
//
//     Ok(Json(json!(site)))
// }

// #[post("/install", data = "<form>")]
// fn post_install(
//     db: DB,
//     sec: State<Sodium>,
//     ip: SocketAddr,
//     l: Lang,
//     form: Json<FmInstall>,
// ) -> Result<Json<Value>> {
//     let db = db.deref();
//     let sec = sec.deref();
//     let Lang(l) = l;
//     let ip = ip.ip();
//
//     try!(validator::email(db, &l, &form.email));
//     try!(validator::username(db, &l, &form.name));
//     try!(validator::password(db, &l, &form.password));
//
//     let tx = try!(db.transaction());
//
//     // check database is empty
//     let count = try!(User::count(&tx));
//     if count > 0 {
//         return e!(
//             ErrorKind::Validator,
//             &tx,
//             &l,
//             s!("nut.errors.database-not-empty"),
//             None::<Value>
//         );
//     }
//     // add email user
//     let user = try!(User::add_by_email(
//         &tx,
//         sec,
//         &form.name,
//         &form.email,
//         &form.password
//     ));
//     l!(
//         &tx,
//         &user,
//         &l,
//         ip,
//         s!("nut.logs.user.sign-up"),
//         None::<Value>
//     );
//     // confirm user
//     try!(User::confirm(&tx, &user));
//     l!(
//         &tx,
//         &user,
//         &l,
//         ip,
//         s!("nut.logs.user.confirm"),
//         None::<Value>
//     );
//     // add roles
//     for n in vec![ROLE_ROOT, ROLE_ADMIN] {
//         let role = try!(Role::get(&tx, &n.to_string(), &None, &None));
//         try!(Policy::apply(&tx, &user, &role, Duration::weeks(100)));
//         l!(
//             &tx,
//             &user,
//             &l,
//             ip,
//             s!("nut.logs.user.apply"),
//             Some(json!({ "name": n }))
//         );
//     }
//
//     try!(tx.commit());
//
//     Ok(Json(json!({})))
// }
