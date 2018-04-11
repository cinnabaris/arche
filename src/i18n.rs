use std::collections::BTreeMap;
use std::ops::Deref;
use std::path::PathBuf;
use std::{ffi, fs};

use chrono::{NaiveDateTime, Utc};
use diesel::connection::Connection as DieselConnection;
use diesel::insert_into;
use diesel::prelude::*;

use ini::Ini;
use log;

use super::orm::Connection;
use super::result::{Error, Result};
use super::schema::locales;

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
    pub fn map_by_lang(con: &Connection, lng: &String) -> Result<BTreeMap<String, String>> {
        let mut items = BTreeMap::new();
        for (code, message) in locales::dsl::locales
            .select((locales::dsl::code, locales::dsl::message))
            .order(locales::dsl::code.asc())
            .filter(locales::dsl::lang.eq(lng))
            .load::<(String, String)>(con.deref())?
        {
            items.insert(code, message);
        }
        Ok(items)
    }

    pub fn sync(con: &Connection, dir: PathBuf) -> Result<(usize, usize)> {
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
