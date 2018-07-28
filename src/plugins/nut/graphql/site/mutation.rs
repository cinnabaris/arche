use std::ops::Deref;

use diesel::Connection as DieselConnection;
use validator::Validate;

use super::super::super::super::super::{
    errors::{Error, Result},
    graphql::{context::Context, H},
    i18n, settings,
};
use super::super::super::consumers::send_mail::Config as SmtpConfig;
use super::models::Author;

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSmtp {
    pub host: String,
    pub port: i32,
    pub user: String,
    pub password: String,
}

impl UpdateSmtp {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        ctx.admin()?;
        let db = ctx.db.deref();
        settings::set(
            db,
            &ctx.app.encryptor,
            &"site.smtp".to_string(),
            &SmtpConfig {
                host: self.host.clone(),
                port: self.port as u16,
                user: self.user.clone(),
                password: self.password.clone(),
            },
            true,
        )?;
        Ok(H::new())
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSeo {
    pub google_site_verify_code: String,
    pub baidu_site_verify_code: String,
}

impl UpdateSeo {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        ctx.admin()?;
        let db = ctx.db.deref();
        db.transaction::<_, Error, _>(|| {
            settings::set(
                db,
                &ctx.app.encryptor,
                &"site.seo.google.site-verify-code".to_string(),
                &self.google_site_verify_code,
                false,
            )?;
            settings::set(
                db,
                &ctx.app.encryptor,
                &"site.seo.baidu.site-verify-code".to_string(),
                &self.baidu_site_verify_code,
                false,
            )?;
            Ok(())
        })?;

        Ok(H::new())
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAuthor {
    #[validate(email, length(min = "2"))]
    pub email: String,
    #[validate(length(min = "1"))]
    pub name: String,
}

impl UpdateAuthor {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        ctx.admin()?;
        let db = ctx.db.deref();
        settings::set(
            db,
            &ctx.app.encryptor,
            &"site.author".to_string(),
            &Author {
                email: self.email.clone(),
                name: self.name.clone(),
            },
            false,
        )?;
        Ok(H::new())
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfo {
    #[validate(length(min = "1"))]
    pub title: String,
    #[validate(length(min = "1"))]
    pub subhead: String,
    #[validate(length(min = "1"))]
    pub keywords: String,
    #[validate(length(min = "1"))]
    pub description: String,
    #[validate(length(min = "1"))]
    pub copyright: String,
}

impl UpdateInfo {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        ctx.admin()?;
        let db = ctx.db.deref();
        db.transaction::<_, Error, _>(|| {
            i18n::set(db, &ctx.locale, &"site.title".to_string(), &self.title)?;
            i18n::set(db, &ctx.locale, &"site.subhead".to_string(), &self.subhead)?;
            i18n::set(
                db,
                &ctx.locale,
                &"site.keywords".to_string(),
                &self.keywords,
            )?;
            i18n::set(
                db,
                &ctx.locale,
                &"site.description".to_string(),
                &self.description,
            )?;
            i18n::set(
                db,
                &ctx.locale,
                &"site.copyright".to_string(),
                &self.copyright,
            )?;
            Ok(())
        })?;

        Ok(H::new())
    }
}
