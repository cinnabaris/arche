use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel::insert_into;
use diesel::prelude::*;
use epub;
use serde_json;

use super::super::super::{
    orm::Connection as Db,
    result::{Error, Result},
    schema::{cbeta_books, cbeta_notes, cbeta_pages},
};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    pub id: i32,
    pub uid: String,
    pub author: String,
    pub publisher: String,
    pub title: String,
    pub mime_type: String,
    pub lang: String,
    pub subject: Option<String>,
    pub description: Option<String>,
    pub published_at: Option<NaiveDate>,
    pub cover: Option<String>,
    pub home: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Book {
    pub fn count(db: &Db) -> Result<i64> {
        let cnt: i64 = cbeta_books::dsl::cbeta_books.count().get_result(db)?;
        Ok(cnt)
    }
    pub fn add(db: &Db, bk: &mut epub::book::Book) -> Result<i32> {
        let mut ct = bk.container()?;
        // get the only opf
        let opf = ct.opf();
        if opf.len() > 1 {
            return Err(Error::WithDescription(format!("multi opf names")));
        }
        if let Some(opf) = opf.first() {
            let opf = bk.opf(opf)?;
            let uid = opf.unique_identifier;
        }
        Ok((0))
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub id: i32,
    pub href: String,
    pub book_id: String,
    pub body: Vec<u8>,
    pub media_type: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Page {
    pub fn count(db: &Db) -> Result<i64> {
        let cnt: i64 = cbeta_pages::dsl::cbeta_pages.count().get_result(db)?;
        Ok(cnt)
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: i32,
    pub user_id: i32,
    pub book_id: i32,
    pub body: String,
    pub media_type: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Note {
    pub fn count(db: &Db, user: &i32) -> Result<i64> {
        let cnt: i64 = cbeta_notes::dsl::cbeta_notes
            .filter(cbeta_notes::dsl::user_id.eq(user))
            .count()
            .get_result(db)?;
        Ok(cnt)
    }
}
