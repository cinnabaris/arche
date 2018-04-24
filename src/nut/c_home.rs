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
