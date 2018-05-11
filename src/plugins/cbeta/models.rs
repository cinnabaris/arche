use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::{delete, insert_into, update};

use super::super::super::{
    orm::Connection as Db,
    result::Result,
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
