use std::ops::Deref;

use diesel::Connection;
use validator::Validate;

use super::super::super::super::{
    errors::{Error, Result},
    graphql::context::Context,
    i18n,
};
use super::super::dao;

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct SignUpUser {
    #[validate(length(min = "1", max = "32"))]
    pub name: String,
    #[validate(length(min = "2", max = "64"))]
    pub email: String,
    #[validate(length(min = "1"))]
    pub password: String,
}

impl SignUpUser {
    pub fn call(&self, ctx: &Context) -> Result<String> {
        self.validate()?;
        let db = ctx.db.deref();
        db.transaction::<_, Error, _>(|| {
            if let Some(user) = dao::add_user_by_email(db, &self.name, &self.email, &self.password)?
            {
                l!(
                    db,
                    &user,
                    &ctx.client_ip,
                    &ctx.locale,
                    "nut.logs.user-sign-up"
                )?;
                return Ok(user.to_string());
            }
            Err(t!(db, &ctx.locale, "nut.errors.user-email-already-exist").into())
        })
    }
}

// https://developers.line.me/en/docs/line-login/web/integrate-line-login/
#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct SignInByLine {
    #[validate(length(min = "2", max = "8"))]
    pub lang: String,
    #[validate(length(min = "1", max = "255"))]
    pub code: String,
    #[validate(length(min = "1"))]
    pub message: String,
}

impl SignInByLine {
    pub fn call(&self, _ctx: &Context) -> Result<String> {
        self.validate()?;
        // TODO
        Ok("".to_string())
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct UpdateLocale {
    #[validate(length(min = "2", max = "8"))]
    pub lang: String,
    #[validate(length(min = "1", max = "255"))]
    pub code: String,
    #[validate(length(min = "1"))]
    pub message: String,
}

impl UpdateLocale {
    pub fn call(&self, ctx: &Context) -> Result<String> {
        self.validate()?;
        ctx.admin()?;
        let db = ctx.db.deref();
        let id = i18n::set(db, &self.lang, &self.code, &self.message)?;
        Ok(id.to_string())
    }
}
