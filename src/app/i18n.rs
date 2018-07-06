use std::collections::BTreeMap;
use std::fs::File;
use std::ops::Deref;
use std::path::PathBuf;
use std::{ffi, fs};

use chrono::Utc;
use diesel::{insert_into, prelude::*, Connection};
use ini::Ini;
use log;
use serde_yaml;

use super::super::{
    context::Context,
    errors::{Error, Result},
    orm::{schema::locales, Connection as Db},
};

pub fn sync(root: PathBuf) -> Result<()> {
    let ctx = Context::new(&super::parse_config()?)?;
    let db = ctx.db.get()?;
    let db = db.deref();
    let (total, inserted) = db.transaction::<_, Error, _>(|| load(db, &root))?;
    log::info!("total {}, insert {}", total, inserted);
    Ok(())
}

fn load(db: &Db, root: &PathBuf) -> Result<(usize, usize)> {
    let mut total = 0;
    let mut inserted = 0;
    let now = Utc::now().naive_utc();
    for (lang, items) in load_from_files(root)? {
        for (code, message) in items {
            total = total + 1;
            if locales::dsl::locales
                .filter(locales::dsl::lang.eq(&lang))
                .filter(locales::dsl::code.eq(&code))
                .count()
                .get_result::<i64>(db)? == 0
            {
                insert_into(locales::dsl::locales)
                    .values((
                        locales::dsl::lang.eq(&lang),
                        locales::dsl::code.eq(&code),
                        locales::dsl::message.eq(&message),
                        locales::dsl::created_at.eq(&now),
                        locales::dsl::updated_at.eq(&now),
                    ))
                    .execute(db)?;
                inserted = inserted + 1;
            }
        }
    }
    Ok((total, inserted))
}
fn parse_locale(file: &PathBuf, ext: &'static str) -> Option<String> {
    if file.extension() == Some(ffi::OsStr::new(ext)) {
        if let Some(name) = file.file_name() {
            if let Some(name) = name.to_str() {
                let lang = &name[..name.len() - ext.len() - 1];
                log::info!("find file {} for locale {}", file.display(), lang);
                return Some(lang.to_string());
            }
        }
    }
    None
}
fn load_from_files(dir: &PathBuf) -> Result<(BTreeMap<String, BTreeMap<String, String>>)> {
    let mut items = BTreeMap::new();
    if dir.is_dir() {
        for it in fs::read_dir(dir)? {
            let it = it?.path();
            if !it.is_dir() {
                if let Some(lang) = parse_locale(&it, "ini") {
                    items.insert(lang, load_from_ini(&it)?);
                } else if let Some(lang) = parse_locale(&it, "yml") {
                    items.insert(lang, load_from_yaml(&it)?);
                } else {
                    log::warn!("unknown filetype {}", it.display());
                }
            }
        }
    }

    return Ok(items);
}
fn parse_yaml_value(p: &Option<String>, v: &serde_yaml::Value) -> BTreeMap<String, String> {
    let mut items = BTreeMap::new();
    if let Some(map) = v.as_mapping() {
        for (k, v) in map {
            if let Some(k) = k.as_str() {
                let p = match p {
                    Some(ref p) => format!("{}.{}", p, k),
                    None => k.to_string(),
                };
                if v.is_mapping() {
                    items.extend(parse_yaml_value(&Some(p), v));
                } else {
                    if let Some(v) = v.as_str() {
                        items.insert(p.clone(), v.to_string());
                    }
                }
            }
        }
    }
    items
}
fn load_from_yaml(file: &PathBuf) -> Result<BTreeMap<String, String>> {
    let fd = File::open(file)?;
    let val: serde_yaml::Value = serde_yaml::from_reader(&fd)?;
    Ok(parse_yaml_value(&None, &val))
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
