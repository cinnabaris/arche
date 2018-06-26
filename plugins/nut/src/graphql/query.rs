use std::ops::Deref;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use validator::Validate;

use super::super::{errors::Result, orm::schema::*, rfc::Utc as ToUtc};
use super::{context::Context, models::Locale};

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
#[graphql(description = "Query locales by lang")]
pub struct LocalesByLang {
    #[validate(length(min = "2", max = "8"))]
    pub lang: String,
}
impl LocalesByLang {
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
