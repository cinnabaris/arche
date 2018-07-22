use std::ops::Deref;

use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use validator::Validate;

use super::super::super::super::{
    errors::Result,
    graphql::{context::Context, H},
    orm::schema::links,
    rfc::UtcDateTime,
};

#[derive(GraphQLObject, Debug, Serialize)]
pub struct Link {
    pub id: String,
    pub href: String,
    pub label: String,
    pub loc: String,
    pub x: i32,
    pub y: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Show {
    #[validate(length(min = "1"))]
    pub id: String,
}

impl Show {
    pub fn call(&self, ctx: &Context) -> Result<Link> {
        self.validate()?;
        let id: i64 = self.id.parse()?;
        let db = ctx.db.deref();
        let (label, href, loc, x, y, updated_at) = links::dsl::links
            .select((
                links::dsl::href,
                links::dsl::label,
                links::dsl::loc,
                links::dsl::x,
                links::dsl::y,
                links::dsl::updated_at,
            ))
            .filter(links::dsl::id.eq(&id))
            .first::<(String, String, String, i16, i16, NaiveDateTime)>(db)?;

        Ok(Link {
            id: self.id.clone(),
            label: label,
            href: href,
            loc: loc,
            x: x as i32,
            y: y as i32,
            updated_at: updated_at.to_utc(),
        })
    }
}
#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Remove {
    #[validate(length(min = "1"))]
    pub id: String,
}

impl Remove {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let id: i64 = self.id.parse()?;
        ctx.admin()?;
        let db = ctx.db.deref();
        let it = links::dsl::links.filter(links::dsl::id.eq(&id));
        delete(it).execute(db)?;
        Ok(H::new())
    }
}

pub fn list(ctx: &Context) -> Result<Vec<Link>> {
    ctx.admin()?;
    let db = ctx.db.deref();
    let items = links::dsl::links
        .select((
            links::dsl::id,
            links::dsl::href,
            links::dsl::label,
            links::dsl::loc,
            links::dsl::x,
            links::dsl::y,
            links::dsl::updated_at,
        ))
        .filter(links::dsl::lang.eq(&ctx.locale))
        .order(links::dsl::loc.asc())
        .order(links::dsl::x.asc())
        .order(links::dsl::y.asc())
        .load::<(i64, String, String, String, i16, i16, NaiveDateTime)>(db)?;

    Ok(items
        .iter()
        .map(|(id, href, label, loc, x, y, updated_at)| Link {
            id: id.to_string(),
            label: label.clone(),
            href: href.clone(),
            loc: loc.clone(),
            x: *x as i32,
            y: *y as i32,
            updated_at: updated_at.to_utc(),
        })
        .collect())
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Create {
    #[validate(length(min = "1"))]
    pub label: String,
    #[validate(length(min = "1"))]
    pub href: String,
    #[validate(length(min = "1"))]
    pub loc: String,
    pub x: i32,
    pub y: i32,
}

impl Create {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let db = ctx.db.deref();
        let now = Utc::now().naive_utc();
        insert_into(links::dsl::links)
            .values((
                links::dsl::lang.eq(&ctx.locale),
                links::dsl::label.eq(&self.label),
                links::dsl::href.eq(&self.href),
                links::dsl::loc.eq(&self.loc),
                links::dsl::x.eq(&(self.x as i16)),
                links::dsl::y.eq(&(self.y as i16)),
                links::dsl::updated_at.eq(&now),
                links::dsl::created_at.eq(&now),
            ))
            .execute(db)?;
        Ok(H::new())
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Update {
    #[validate(length(min = "1"))]
    pub id: String,
    #[validate(length(min = "1"))]
    pub label: String,
    #[validate(length(min = "1"))]
    pub href: String,
    #[validate(length(min = "1"))]
    pub loc: String,
    pub x: i32,
    pub y: i32,
}

impl Update {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let id = self.id.parse::<i64>()?;
        let db = ctx.db.deref();
        let now = Utc::now().naive_utc();
        let it = links::dsl::links.filter(links::dsl::id.eq(&id));
        update(it)
            .set((
                links::dsl::label.eq(&self.label),
                links::dsl::href.eq(&self.href),
                links::dsl::loc.eq(&self.loc),
                links::dsl::x.eq(&(self.x as i16)),
                links::dsl::y.eq(&(self.y as i16)),
                links::dsl::updated_at.eq(&now),
            ))
            .execute(db)?;
        Ok(H::new())
    }
}
