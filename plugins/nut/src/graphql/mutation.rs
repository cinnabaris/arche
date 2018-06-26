use std::ops::Deref;

use validator::Validate;

use super::super::{errors::Result, i18n::Dao as I18nDao, orm::Dao};
use super::context::Context;

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
    pub fn call(&self, ctx: &Context) -> Result<Option<String>> {
        self.validate()?;
        ctx.admin()?;
        let db = Dao::new(ctx.db.deref());
        let id = db.set(&self.lang, &self.code, &self.message, true)?;
        if let Some(id) = id {
            return Ok(Some(id.to_string()));
        }
        Ok(None)
    }
}
