use std::ops::Deref;

use validator::Validate;

use super::super::super::super::{errors::Result, graphql::context::Context, i18n};

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
        let db = ctx.state.db.get()?;
        let db = db.deref();
        let id = i18n::set(db, &self.lang, &self.code, &self.message)?;
        Ok(id.to_string())
    }
}
