use std::ops::Deref;

use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use validator::Validate;

use super::super::super::super::{
    errors::Result,
    graphql::{context::Context, H},
    orm::schema::cards,
    rfc::UtcDateTime,
};

#[derive(GraphQLObject, Debug, Serialize)]
pub struct Card {
    pub id: String,
    pub title: String,
    pub body: String,
    pub media_type: String,
    pub action: String,
    pub href: String,
    pub logo: String,
    pub loc: String,
    pub position: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Show {
    #[validate(length(min = "1"))]
    pub id: String,
}

impl Show {
    pub fn call(&self, ctx: &Context) -> Result<Card> {
        self.validate()?;
        let id: i64 = self.id.parse()?;
        let db = ctx.db.deref();
        let (title, body, media_type, action, href, logo, loc, position, updated_at) =
            cards::dsl::cards
                .select((
                    cards::dsl::title,
                    cards::dsl::body,
                    cards::dsl::media_type,
                    cards::dsl::action,
                    cards::dsl::href,
                    cards::dsl::logo,
                    cards::dsl::loc,
                    cards::dsl::position,
                    cards::dsl::updated_at,
                ))
                .filter(cards::dsl::id.eq(&id))
                .first::<(
                    String,
                    String,
                    String,
                    String,
                    String,
                    String,
                    String,
                    i16,
                    NaiveDateTime,
                )>(db)?;

        Ok(Card {
            id: self.id.clone(),
            title: title,
            body: body,
            media_type: media_type,
            action: action,
            href: href,
            logo: logo,
            loc: loc,
            position: position as i32,
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
        let it = cards::dsl::cards.filter(cards::dsl::id.eq(&id));
        delete(it).execute(db)?;
        Ok(H::new())
    }
}

pub fn list(ctx: &Context) -> Result<Vec<Card>> {
    ctx.admin()?;
    let db = ctx.db.deref();
    let items = cards::dsl::cards
        .select((
            cards::dsl::id,
            cards::dsl::title,
            cards::dsl::body,
            cards::dsl::media_type,
            cards::dsl::action,
            cards::dsl::href,
            cards::dsl::logo,
            cards::dsl::loc,
            cards::dsl::position,
            cards::dsl::updated_at,
        ))
        .filter(cards::dsl::lang.eq(&ctx.locale))
        .order(cards::dsl::loc.asc())
        .order(cards::dsl::position.asc())
        .load::<(
            i64,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            i16,
            NaiveDateTime,
        )>(db)?;

    Ok(items
        .iter()
        .map(
            |(id, title, body, media_type, action, href, logo, loc, position, updated_at)| Card {
                id: id.to_string(),
                title: title.clone(),
                body: body.clone(),
                media_type: media_type.clone(),
                action: action.clone(),
                href: href.clone(),
                logo: logo.clone(),
                loc: loc.clone(),
                position: *position as i32,
                updated_at: updated_at.to_utc(),
            },
        )
        .collect())
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Create {
    #[validate(length(min = "1"))]
    pub title: String,
    #[validate(length(min = "1"))]
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
    #[validate(length(min = "1"))]
    pub action: String,
    #[validate(length(min = "1"))]
    pub href: String,
    #[validate(length(min = "1"))]
    pub logo: String,
    #[validate(length(min = "1"))]
    pub loc: String,
    pub position: i32,
}

impl Create {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let db = ctx.db.deref();
        let now = Utc::now().naive_utc();
        insert_into(cards::dsl::cards)
            .values((
                cards::dsl::lang.eq(&ctx.locale),
                cards::dsl::title.eq(&self.title),
                cards::dsl::body.eq(&self.body),
                cards::dsl::media_type.eq(&self.media_type),
                cards::dsl::action.eq(&self.action),
                cards::dsl::href.eq(&self.href),
                cards::dsl::logo.eq(&self.logo),
                cards::dsl::loc.eq(&self.loc),
                cards::dsl::position.eq(&(self.position as i16)),
                cards::dsl::updated_at.eq(&now),
                cards::dsl::created_at.eq(&now),
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
    pub title: String,
    #[validate(length(min = "1"))]
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
    #[validate(length(min = "1"))]
    pub action: String,
    #[validate(length(min = "1"))]
    pub href: String,
    #[validate(length(min = "1"))]
    pub logo: String,
    #[validate(length(min = "1"))]
    pub loc: String,
    pub position: i32,
}

impl Update {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let id = self.id.parse::<i64>()?;
        let db = ctx.db.deref();
        let now = Utc::now().naive_utc();
        let it = cards::dsl::cards.filter(cards::dsl::id.eq(&id));
        update(it)
            .set((
                cards::dsl::title.eq(&self.title),
                cards::dsl::body.eq(&self.body),
                cards::dsl::media_type.eq(&self.media_type),
                cards::dsl::action.eq(&self.action),
                cards::dsl::href.eq(&self.href),
                cards::dsl::logo.eq(&self.logo),
                cards::dsl::loc.eq(&self.loc),
                cards::dsl::position.eq(&(self.position as i16)),
                cards::dsl::updated_at.eq(&now),
            ))
            .execute(db)?;
        Ok(H::new())
    }
}
