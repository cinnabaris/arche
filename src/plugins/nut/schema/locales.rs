use std::ops::Deref;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use validator::Validate;

use super::super::super::super::{graphql::context::Context, result::Result, schema::locales};

#[derive(Serialize, GraphQLInputObject, Deserialize, Debug, Validate)]
#[graphql(description = "Search locales by lang")]
pub struct FmByLang {
    #[validate(length(min = "2", max = "8"))]
    pub lang: String,
}

impl FmByLang {
    pub fn call(&self, ctx: &Context) -> Result<Vec<Locale>> {
        let db = ctx.db.deref();
        let items = locales::dsl::locales
            .select((
                locales::dsl::id,
                locales::dsl::code,
                locales::dsl::message,
                locales::dsl::updated_at,
            ))
            .filter(locales::dsl::lang.eq(&self.lang))
            .get_results(db)?;
        Ok(items)
    }
}

#[derive(Queryable, GraphQLObject, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "Locale mode for translation")]
pub struct Locale {
    pub id: i32,
    pub code: String,
    pub message: String,
    pub updated_at: NaiveDateTime,
}
