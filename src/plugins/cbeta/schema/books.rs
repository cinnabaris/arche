use std::ops::Deref;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use validator::Validate;

use super::super::super::super::{graphql::context::Context, result::Result, schema::cbeta_books};
use super::super::models;

#[derive(Serialize, GraphQLInputObject, Deserialize, Debug, Validate)]
#[graphql(description = "Search cbeta.book by id")]
pub struct FmById {
    pub id: i32,
}

impl FmById {
    pub fn call(&self, ctx: &Context) -> Result<models::Book> {
        let db = ctx.db.deref();
        models::Book::get(db, &self.id)
    }
}

#[derive(Queryable, GraphQLObject, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub creator: String,
    pub published_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

pub fn all(ctx: &Context) -> Result<Vec<Book>> {
    let db = ctx.db.deref();
    let items = cbeta_books::dsl::cbeta_books
        .select((
            cbeta_books::dsl::id,
            cbeta_books::dsl::title,
            cbeta_books::dsl::creator,
            cbeta_books::dsl::published_at,
            cbeta_books::dsl::updated_at,
        ))
        .get_results(db)?;
    Ok(items)
}
