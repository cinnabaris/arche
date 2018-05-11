use std::ffi::OsStr;
use std::fs::{read_dir, File};
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use diesel::Connection;
use log;
use serde_json;

use super::super::super::{
    orm::Connection as Db,
    result::{Error, Result},
};
use super::models::{Book, Page};

pub fn load(db: &Db) -> Result<()> {
    books(db, Path::new("tmp").join("cbeta"))?;
    Ok(())
}

fn books(db: &Db, root: PathBuf) -> Result<()> {
    if root.is_dir() {
        log::info!("load books from {:?}", root);
        for it in read_dir(root)? {
            let it = it?.path();
            if it.extension() == Some(OsStr::new("json")) {
                log::info!("find book {:?}", it);
                parse(db, it)?;
            }
        }
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Epub {
    title: String,
    identifier: String,
    language: String,
    creator: String,
    publisher: Option<String>,
    subject: Option<String>,
    description: Option<String>,
    updated_at: Option<DateTime<Utc>>,
    version: String,
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Item {
    href: String,
    id: String,
    media_type: String,
    content: Option<Vec<u8>>,
}

fn parse(db: &Db, file: PathBuf) -> Result<()> {
    let fd = File::open(file)?;
    let it: Epub = serde_json::from_reader(fd)?;
    db.transaction::<_, Error, _>(|| {
        let bid = Book::set(
            db,
            &it.title,
            &it.identifier,
            &it.language,
            &it.creator,
            &it.publisher,
            &it.subject,
            &it.description,
            &match it.updated_at {
                Some(v) => Some(v.naive_utc()),
                None => None,
            },
            &it.version,
        )?;
        let n = Page::clear(db, &bid)?;
        log::info!("remove {} pages", n);
        for it in it.items {
            Page::add(db, &bid, &it.id, &it.href, &it.content, &it.media_type)?;
        }
        Ok(())
    })
}
