use std::ops::Deref;

use super::super::super::super::super::{
    env, errors::Result, graphql::context::Context, i18n, settings, sys,
};
use super::super::super::consumers::send_mail::Config as SmtpConfig;
use super::models::{Author, Info, Seo, Smtp, Status};

pub fn smtp(ctx: &Context) -> Result<Smtp> {
    ctx.admin()?;
    let db = ctx.db.deref();
    let it: SmtpConfig = settings::get(db, &ctx.app.encryptor, &"site.smtp".to_string())?;
    Ok(Smtp {
        host: it.host,
        port: it.port as i32,
        user: it.user,
        password: "".to_string(),
    })
}

pub fn seo(ctx: &Context) -> Result<Seo> {
    ctx.admin()?;
    let db = ctx.db.deref();
    Ok(Seo {
        google_site_verify_code: settings::get(
            db,
            &ctx.app.encryptor,
            &"site.seo.google.site-verify-code".to_string(),
        )?,
        baidu_site_verify_code: settings::get(
            db,
            &ctx.app.encryptor,
            &"site.seo.baidu.site-verify-code".to_string(),
        )?,
    })
}

pub fn author(ctx: &Context) -> Result<Author> {
    let db = ctx.db.deref();
    settings::get(db, &ctx.app.encryptor, &"site.author".to_string())
}

pub fn info(ctx: &Context) -> Result<Info> {
    let db = ctx.db.deref();
    Ok(Info {
        title: t!(db, &ctx.locale, "site.title"),
        subhead: t!(db, &ctx.locale, "site.subhead"),
        keywords: t!(db, &ctx.locale, "site.keywords"),
        description: t!(db, &ctx.locale, "site.description"),
        copyright: t!(db, &ctx.locale, "site.copyright"),
        languages: i18n::languages(db)?,
    })
}

pub fn status(ctx: &Context) -> Result<Vec<Status>> {
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
