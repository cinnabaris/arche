use std::ops::Deref;

use super::super::super::super::super::{
    env, errors::Result, graphql::context::Context, i18n, settings, sys,
};
use super::models::{Author, Info, Status};

pub fn get_author(ctx: &Context) -> Result<Author> {
    let db = ctx.db.deref();
    settings::get(db, &ctx.app.encryptor, &"site.author".to_string())
}

pub fn get_info(ctx: &Context) -> Result<Info> {
    let db = ctx.db.deref();
    Ok(Info {
        title: t!(db, &ctx.locale, "site.title"),
        subhead: t!(db, &ctx.locale, "site.subhead"),
        keywords: t!(db, &ctx.locale, "site.keywords"),
        description: t!(db, &ctx.locale, "site.description"),
        copyright: t!(db, &ctx.locale, "site.copyright"),
    })
}

pub fn get_status(ctx: &Context) -> Result<Vec<Status>> {
    ctx.admin()?;
    let db = ctx.db.deref();

    let mut items = Vec::new();
    items.push(Status {
        name: "os".to_string(),
        value: sys::version()?,
    });
    items.push(Status {
        name: "hostname".to_string(),
        value: sys::hostname()?,
    });
    items.push(Status {
        name: "pid".to_string(),
        value: sys::pid().to_string(),
    });
    items.push(Status {
        name: "app version".to_string(),
        value: env::version(),
    });
    items.push(Status {
        name: "app languages".to_string(),
        value: i18n::languages(db)?.join(", "),
    });
    items.push(Status {
        name: "app authors".to_string(),
        value: env::AUTHORS.to_string(),
    });
    items.push(Status {
        name: "app homepage".to_string(),
        value: env::HOMEPAGE.to_string(),
    });
    Ok(items)
}
