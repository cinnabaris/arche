use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use std::str::FromStr;
use std::{ffi, fs};

use chrono::Utc;
use diesel::prelude::*;
use diesel::{insert_into, update};
use handlebars::Handlebars;
use hyper::header::{AcceptLanguage, Header, LanguageTag, Raw};
use ini::Ini;
use log;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use serde::ser::Serialize;
use url::Url;

use super::{
    env,
    errors::{Error, Result},
};

use super::orm::{self, schema::locales};

pub trait Dao {
    fn get(&self, lang: &String, code: &String) -> Result<String>;
    fn set(
        &self,
        lang: &String,
        code: &String,
        message: &String,
        override_: bool,
    ) -> Result<Option<i64>>;
    fn by_lang(&self, lang: &String) -> Result<BTreeMap<String, String>>;
}

impl<'a> Dao for orm::Dao<'a> {
    fn get(&self, lang: &String, code: &String) -> Result<String> {
        let msg = locales::dsl::locales
            .select(locales::dsl::message)
            .filter(locales::dsl::lang.eq(lang))
            .filter(locales::dsl::code.eq(code))
            .first::<String>(self.db)?;
        Ok(msg)
    }
    fn set(
        &self,
        lang: &String,
        code: &String,
        message: &String,
        override_: bool,
    ) -> Result<Option<i64>> {
        let now = Utc::now().naive_utc();
        match locales::dsl::locales
            .select(locales::dsl::id)
            .filter(locales::dsl::lang.eq(lang))
            .filter(locales::dsl::code.eq(code))
            .first::<i64>(self.db)
        {
            Ok(id) => {
                if !override_ {
                    return Ok(None);
                }
                let it = locales::dsl::locales.filter(locales::dsl::id.eq(&id));
                update(it)
                    .set((
                        locales::dsl::message.eq(message),
                        locales::dsl::updated_at.eq(&now),
                    ))
                    .execute(self.db)?;
                Ok(Some(id))
            }
            Err(_) => {
                let id = insert_into(locales::dsl::locales)
                    .values((
                        locales::dsl::lang.eq(lang),
                        locales::dsl::code.eq(code),
                        locales::dsl::message.eq(message),
                        locales::dsl::updated_at.eq(&now),
                    ))
                    .returning(locales::dsl::id)
                    .get_result::<i64>(self.db)?;
                Ok(Some(id))
            }
        }
    }
    fn by_lang(&self, lang: &String) -> Result<BTreeMap<String, String>> {
        let mut items = BTreeMap::new();
        for (code, message) in locales::dsl::locales
            .select((locales::dsl::code, locales::dsl::message))
            .order(locales::dsl::code.asc())
            .filter(locales::dsl::lang.eq(lang))
            .load::<(String, String)>(self.db)?
        {
            items.insert(code, message);
        }
        Ok(items)
    }
}

//-----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Params(pub HashMap<String, String>);

impl<'a, 'r> FromRequest<'a, 'r> for Params {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Params, ()> {
        match Url::parse(&format!("{}{}", "fake:/", req.uri().as_str())) {
            Ok(u) => Outcome::Success(Params(u.query_pairs().into_owned().collect())),
            _ => Outcome::Failure((Status::BadRequest, ())),
        }
    }
}

//-----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Locale {
    // https://tools.ietf.org/html/bcp47
    pub name: String,
}

impl Locale {
    fn parse<'a, 'r>(req: &'a Request<'r>) -> Result<Option<LanguageTag>> {
        let key = String::from("locale");
        // 1. Check URL arguments.
        if let Outcome::Success(Params(it)) = req.guard::<Params>() {
            if let Some(l) = it.get(&key) {
                return Ok(Some(LanguageTag::from_str(l)?));
            }
        }
        // 2. Get language information from cookies.
        if let Some(ck) = req.cookies().get(&key[..]) {
            return Ok(Some(LanguageTag::from_str(ck.value())?));
        }
        // 3. Get language information from 'Accept-Language'.
        // https://www.w3.org/International/questions/qa-accept-lang-locales
        // https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.4
        if let Some(al) = req.headers().get_one(AcceptLanguage::header_name()) {
            if let Ok(al) = AcceptLanguage::parse_header(&Raw::from(al)) {
                if let Some(al) = al.first() {
                    return Ok(Some(al.item.clone()));
                }
            }
        }
        return Ok(None);
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Locale {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
        if let Outcome::Success(cfg) = req.guard::<State<env::Config>>() {
            if let Ok(l) = Self::parse(req) {
                if let Some(l) = l {
                    for it in &cfg.languages {
                        if let Ok(ref t) = LanguageTag::from_str(&it[..]) {
                            if l.matches(t) {
                                return Outcome::Success(Self {
                                    name: it.to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }
        return Outcome::Success(Self {
            name: "en-US".into(),
        });
    }
}

//-----------------------------------------------------------------------------

pub fn e<S: Serialize, D: Dao>(db: &D, lang: &String, code: &String, args: Option<S>) -> Error {
    t(db, lang, code, args).into()
}

pub fn t<S: Serialize, D: Dao>(db: &D, lang: &String, code: &String, args: Option<S>) -> String {
    if let Ok(msg) = db.get(lang, code) {
        if let Some(args) = args {
            if let Ok(msg) = Handlebars::new().render_template(&msg, &args) {
                return msg;
            }
        } else {
            return msg;
        }
    }
    return format!("{}.{}", lang, code);
}

pub fn sync<D: Dao>(db: &D, dir: PathBuf) -> Result<(usize, usize)> {
    let mut total = 0;
    let mut inserted = 0;
    for (lang, items) in load_from_files(dir)? {
        for (code, message) in items {
            total = total + 1;
            if let Some(_id) = db.set(&lang, &code, &message, false)? {
                inserted = inserted + 1;
            }
        }
    }
    Ok((total, inserted))
}

fn load_from_files(dir: PathBuf) -> Result<(BTreeMap<String, BTreeMap<String, String>>)> {
    let mut items = BTreeMap::new();
    if dir.is_dir() {
        for it in fs::read_dir(dir)? {
            let it = it?.path();
            if !it.is_dir() {
                if it.extension() == Some(ffi::OsStr::new(INI)) {
                    if let Some(name) = it.file_name() {
                        if let Some(name) = name.to_str() {
                            let lang = &name[..name.len() - 4];
                            log::info!("find locale {}", lang);
                            items.insert(String::from(lang), load_from_ini(&it)?);
                        }
                    }
                }
            }
        }
    }

    return Ok(items);
}
fn load_from_ini(file: &PathBuf) -> Result<BTreeMap<String, String>> {
    let mut items = BTreeMap::new();
    for (sec, prop) in Ini::load_from_file(file)?.iter() {
        for (k, v) in prop.iter() {
            let mut key = String::new();
            match *sec {
                Some(ref sec) => {
                    key = key + &sec;
                    key.push_str(".");
                }
                None => {}
            }
            key = key + k;
            items.insert(key, String::new() + v);
        }
    }
    return Ok(items);
}

pub const INI: &'static str = "ini";
