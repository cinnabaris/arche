use std::io::Cursor;
use std::str::FromStr;

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::{delete, insert_into, update};
use log;
use rocket::http::{ContentType, Status};
use rocket::response::{self, Responder};
use rocket::{Request, Response};

use super::super::super::{
    format::Pager,
    orm::Connection as Db,
    result::{Error, Result},
    schema::{cbeta_books, cbeta_notes, cbeta_pages},
};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub identifier: String,
    pub language: String,
    pub creator: String,
    pub publisher: Option<String>,
    pub subject: Option<String>,
    pub description: Option<String>,
    pub published_at: Option<NaiveDateTime>,
    pub version: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Book {
    pub fn list(db: &Db, pager: &Pager) -> Result<(i64, Vec<Self>)> {
        let total = Self::count(db)?;
        let (limit, offset) = pager.build(total);
        let items = cbeta_books::dsl::cbeta_books
            .offset(offset)
            .limit(limit)
            .order(cbeta_books::dsl::title.asc())
            .load(db)?;
        Ok((total, items))
    }
    pub fn get(db: &Db, id: &i32) -> Result<Self> {
        let it = cbeta_books::dsl::cbeta_books
            .filter(cbeta_books::dsl::id.eq(id))
            .first(db)?;
        Ok(it)
    }

    pub fn count(db: &Db) -> Result<i64> {
        let cnt: i64 = cbeta_books::dsl::cbeta_books.count().get_result(db)?;
        Ok(cnt)
    }

    pub fn set(
        db: &Db,
        title: &String,
        identifier: &String,
        language: &String,
        creator: &String,
        publisher: &Option<String>,
        subject: &Option<String>,
        description: &Option<String>,
        published_at: &Option<NaiveDateTime>,
        version: &String,
    ) -> Result<i32> {
        let now = Utc::now().naive_utc();
        match cbeta_books::dsl::cbeta_books
            .select(cbeta_books::dsl::id)
            .filter(cbeta_books::dsl::identifier.eq(identifier))
            .first::<i32>(db)
        {
            Ok(id) => {
                let it = cbeta_books::dsl::cbeta_books.filter(cbeta_books::dsl::id.eq(&id));
                update(it)
                    .set((
                        cbeta_books::dsl::title.eq(title),
                        cbeta_books::dsl::language.eq(language),
                        cbeta_books::dsl::creator.eq(creator),
                        cbeta_books::dsl::publisher.eq(publisher),
                        cbeta_books::dsl::subject.eq(subject),
                        cbeta_books::dsl::description.eq(description),
                        cbeta_books::dsl::published_at.eq(published_at),
                        cbeta_books::dsl::version.eq(version),
                        cbeta_books::dsl::updated_at.eq(&now),
                    ))
                    .execute(db)?;
                Ok(id)
            }
            Err(_) => {
                let id = insert_into(cbeta_books::dsl::cbeta_books)
                    .values((
                        cbeta_books::dsl::identifier.eq(identifier),
                        cbeta_books::dsl::title.eq(title),
                        cbeta_books::dsl::language.eq(language),
                        cbeta_books::dsl::creator.eq(creator),
                        cbeta_books::dsl::publisher.eq(publisher),
                        cbeta_books::dsl::subject.eq(subject),
                        cbeta_books::dsl::description.eq(description),
                        cbeta_books::dsl::published_at.eq(published_at),
                        cbeta_books::dsl::version.eq(version),
                        cbeta_books::dsl::updated_at.eq(&now),
                    ))
                    .returning(cbeta_books::dsl::id)
                    .get_result::<i32>(db)?;
                Ok(id)
            }
        }
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub id: i32,
    pub book_id: i32,
    pub name: String,
    pub href: String,
    pub body: Option<Vec<u8>>,
    pub media_type: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Page {
    pub fn home(db: &Db, book: &i32) -> Result<Self> {
        // let ver: String = cbeta_books::dsl::cbeta_books
        //     .select(cbeta_books::dsl::version)
        //     .filter(cbeta_books::dsl::id.eq(book))
        //     .first(db)?;
        // let name = match &ver[..] {
        //     "2.0" => Ok("ncx"),
        //     "3.0" => Ok("toc"),
        //     _ => Err(Error::WithDescription(format!("bad version {}", ver))),
        // }?;

        for name in vec!["toc", "ncx"] {
            if let Ok(it) = cbeta_pages::dsl::cbeta_pages
                .filter(cbeta_pages::dsl::book_id.eq(book))
                .filter(cbeta_pages::dsl::name.eq(name))
                .first(db)
            {
                return Ok(it);
            }
        }
        Err(Error::WithDescription(format!("can't find toc")))
    }
    pub fn get(db: &Db, book: &i32, href: &String) -> Result<Self> {
        let it = cbeta_pages::dsl::cbeta_pages
            .filter(cbeta_pages::dsl::book_id.eq(book))
            .filter(cbeta_pages::dsl::href.eq(href))
            .first(db)?;
        Ok(it)
    }
    pub fn count(db: &Db) -> Result<i64> {
        let cnt: i64 = cbeta_pages::dsl::cbeta_pages.count().get_result(db)?;
        Ok(cnt)
    }

    pub fn clear(db: &Db, book: &i32) -> Result<usize> {
        let it = cbeta_pages::dsl::cbeta_pages.filter(cbeta_pages::dsl::book_id.eq(book));
        let num = delete(it).execute(db)?;
        Ok(num)
    }

    pub fn add(
        db: &Db,
        book: &i32,
        name: &String,
        href: &String,
        body: &Option<Vec<u8>>,
        media_type: &String,
    ) -> Result<i32> {
        let now = Utc::now().naive_utc();
        let id = insert_into(cbeta_pages::dsl::cbeta_pages)
            .values((
                cbeta_pages::dsl::book_id.eq(book),
                cbeta_pages::dsl::name.eq(name),
                cbeta_pages::dsl::href.eq(href),
                cbeta_pages::dsl::body.eq(body),
                cbeta_pages::dsl::media_type.eq(media_type),
                cbeta_pages::dsl::updated_at.eq(&now),
            ))
            .returning(cbeta_pages::dsl::id)
            .get_result::<i32>(db)?;
        Ok(id)
    }
}

impl<'r> Responder<'r> for Page {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        // https://api.rocket.rs/rocket/http/struct.ContentType.html
        // select distinct media_type from cbeta_pages;
        let mt = match &self.media_type[..] {
            "image/gif" => ContentType::GIF,
            "text/css" => ContentType::CSS,
            "image/jpeg" => ContentType::JPEG,
            "image/svg" => ContentType::SVG,
            "application/xhtml+xml" => ContentType::HTML,
            "application/adobe-page-template+xml" | "application/x-dtbncx+xml" => ContentType::XML,
            t => {
                log::warn!("detect media-type for {}", t);
                match ContentType::from_str(t) {
                    Ok(t) => t,
                    Err(e) => {
                        log::warn!("{:?}", e);
                        ContentType::Plain
                    }
                }
            }
        };
        match self.body {
            Some(body) => Response::build()
                .header(mt)
                .sized_body(Cursor::new(body))
                .ok(),
            None => Response::build()
                .header(ContentType::Plain)
                .status(Status::NotFound)
                .ok(),
        }

        // match self.media_type {
        //     Blog::File(f) => f.respond_to(req),
        //     Blog::Html(t) => t.respond_to(req),
        // }
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
