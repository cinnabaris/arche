use std::ops::Deref;

use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{delete, prelude::*};
use validator::Validate;

use super::super::super::super::{
    errors::Result,
    graphql::{context::Context, H},
    i18n,
    orm::schema::*,
    rfc::Utc as ToUtc,
};

#[derive(GraphQLObject, Debug, Serialize)]
#[graphql(description = "Message for translate")]
pub struct Locale {
    pub id: String,
    pub lang: String,
    pub code: String,
    pub message: String,
    pub updated_at: DateTime<Utc>,
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
        let it = locales::dsl::locales.filter(locales::dsl::id.eq(&id));
        delete(it).execute(db)?;
        Ok(H::new())
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Update {
    #[validate(length(min = "1", max = "255"))]
    pub code: String,
    #[validate(length(min = "1"))]
    pub message: String,
}

impl Update {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        ctx.admin()?;
        let db = ctx.db.deref();
        i18n::set(db, &ctx.locale, &self.code, &self.message)?;
        Ok(H::new())
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Get {
    #[validate(length(min = "1"))]
    pub code: String,
}

impl Get {
    pub fn call(&self, ctx: &Context) -> Result<Locale> {
        self.validate()?;
        let db = ctx.db.deref();
        let (id, message, updated_at) = locales::dsl::locales
            .select((
                locales::dsl::id,
                locales::dsl::message,
                locales::dsl::updated_at,
            ))
            .filter(locales::dsl::lang.eq(&ctx.locale))
            .filter(locales::dsl::code.eq(&self.code))
            .first::<(i64, String, NaiveDateTime)>(db)?;

        Ok(Locale {
            id: id.to_string(),
            lang: ctx.locale.clone(),
            code: self.code.clone(),
            message: message.clone(),
            updated_at: updated_at.to_utc(),
        })
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
#[graphql(description = "Query locales by lang")]
pub struct ListByLang {
    #[validate(length(min = "2", max = "8"))]
    pub lang: String,
}

impl ListByLang {
    pub fn call(&self, ctx: &Context) -> Result<Vec<Locale>> {
        self.validate()?;
        let db = ctx.db.deref();
        let items = locales::dsl::locales
            .select((
                locales::dsl::id,
                locales::dsl::lang,
                locales::dsl::code,
                locales::dsl::message,
                locales::dsl::updated_at,
            ))
            .order(locales::dsl::code.asc())
            .filter(locales::dsl::lang.eq(&self.lang))
            .load::<(i64, String, String, String, NaiveDateTime)>(db)?;

        Ok(items
            .iter()
            .map(|(i, l, c, m, u)| Locale {
                id: i.to_string(),
                lang: l.clone(),
                code: c.clone(),
                message: m.clone(),
                updated_at: u.to_utc(),
            })
            .collect())
    }
}
