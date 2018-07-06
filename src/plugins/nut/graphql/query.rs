use std::ops::Deref;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use validator::Validate;

use super::super::super::super::{
    errors::Result, graphql::context::Context, i18n, orm::schema::*, rfc::Utc as ToUtc, settings,
};
use super::models::{Locale, Oauth, SiteInfo};

pub fn get_site_info(ctx: &Context) -> Result<SiteInfo> {
    let db = ctx.db.deref();
    Ok(SiteInfo {
        locale: ctx.locale.clone(),
        title: i18n::t(
            &db,
            &ctx.locale,
            &String::from("site.title"),
            &None::<String>,
        ),
        subhead: i18n::t(
            &db,
            &ctx.locale,
            &String::from("site.subhead"),
            &None::<String>,
        ),
        keywords: i18n::t(
            &db,
            &ctx.locale,
            &String::from("site.keywords"),
            &None::<String>,
        ),
        description: i18n::t(
            &db,
            &ctx.locale,
            &String::from("site.description"),
            &None::<String>,
        ),
        copyright: i18n::t(
            &db,
            &ctx.locale,
            &String::from("site.copyright"),
            &None::<String>,
        ),
        author: settings::get(&db, &ctx.encryptor, &String::from("site.author"))?,
        oauth: Oauth::new(&ctx.config.oauth),
    })
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
#[graphql(description = "Query locales by lang")]
pub struct ListLocaleByLang {
    #[validate(length(min = "2", max = "8"))]
    pub lang: String,
}

impl ListLocaleByLang {
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
