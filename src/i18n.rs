use std::collections::{BTreeMap, HashMap};
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;
use std::{ffi, fs};

use chrono::{NaiveDateTime, Utc};
use diesel::connection::Connection as DieselDb;
use diesel::insert_into;
use diesel::prelude::*;
use handlebars::Handlebars;
use hyper::header::{AcceptLanguage, Header, LanguageTag, Raw};
use ini::Ini;
use log;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use serde::ser::Serialize;
use url::Url;

use super::env;
use super::orm::Connection as Db;
use super::result::{Error, Result};
use super::schema::locales;

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
pub struct Lang(pub String);

impl Lang {
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

impl<'a, 'r> FromRequest<'a, 'r> for Lang {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
        if let Outcome::Success(cfg) = req.guard::<State<env::Config>>() {
            if let Ok(l) = Self::parse(req) {
                if let Some(l) = l {
                    for it in &cfg.languages {
                        if let Ok(ref t) = LanguageTag::from_str(&it[..]) {
                            if l.matches(t) {
                                return Outcome::Success(Lang(it.to_string()));
                            }
                        }
                    }
                }
            }
        }
        return Outcome::Success(Lang("en-US".into()));
    }
}

//-----------------------------------------------------------------------------

// https://tools.ietf.org/html/bcp47

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Locale {
    pub id: i64,
    pub lang: String,
    pub code: String,
    pub message: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: NaiveDateTime,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
}

impl Locale {
    pub fn e<T: Serialize>(db: &Db, lang: &String, code: &String, args: Option<T>) -> Error {
        Error::WithDescription(Locale::t(db, lang, code, args))
    }
    pub fn t<T: Serialize>(db: &Db, lang: &String, code: &String, args: Option<T>) -> String {
        if let Ok(msg) = Locale::get_message(db, lang, code) {
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
    pub fn get_message(con: &Db, lang: &String, code: &String) -> Result<String> {
        let msg = locales::dsl::locales
            .select(locales::dsl::message)
            .filter(locales::dsl::lang.eq(lang))
            .filter(locales::dsl::code.eq(code))
            .first::<String>(con.deref())?;
        Ok(msg)
    }
    pub fn map_by_lang(con: &Db, lang: &String) -> Result<BTreeMap<String, String>> {
        let mut items = BTreeMap::new();
        for (code, message) in locales::dsl::locales
            .select((locales::dsl::code, locales::dsl::message))
            .order(locales::dsl::code.asc())
            .filter(locales::dsl::lang.eq(lang))
            .load::<(String, String)>(con.deref())?
        {
            items.insert(code, message);
        }
        Ok(items)
    }

    pub fn sync(con: &Db, dir: PathBuf) -> Result<(usize, usize)> {
        let con = con.deref();
        con.transaction::<_, Error, _>(|| {
            let mut total = 0;
            let mut inserted = 0;
            for (lang, items) in Locale::load_from_files(dir)? {
                for (code, message) in items {
                    total = total + 1;
                    let cnt: i64 = locales::dsl::locales
                        .filter(locales::dsl::lang.eq(&lang))
                        .filter(locales::dsl::code.eq(&code))
                        .count()
                        .get_result(con)?;
                    if cnt == 0 {
                        inserted = inserted + 1;
                        insert_into(locales::dsl::locales)
                            .values((
                                locales::dsl::lang.eq(&lang),
                                locales::dsl::code.eq(&code),
                                locales::dsl::message.eq(&message),
                                locales::dsl::updated_at.eq(Utc::now().naive_utc()),
                            ))
                            .execute(con)?;
                    }
                }
            }
            Ok((total, inserted))
        })
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
                                items.insert(s!(lang), Locale::load_from_ini(&it)?);
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
}

pub const INI: &'static str = "ini";
