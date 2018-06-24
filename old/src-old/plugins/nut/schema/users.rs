use std::ops::Deref;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use validator::Validate;

use super::super::super::super::{
    graphql::{context::Context, H}, result::Result, schema::{logs, policies, roles, users},
};

//-----------------------------------------------------------------------------

#[derive(Serialize, GraphQLInputObject, Deserialize, Debug, Validate)]
#[graphql(description = "Create a new account")]
pub struct FmSignUp {
    #[validate(length(min = "2", max = "64"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "6", max = "32"))]
    pub password: String,
}

impl FmSignUp {
    pub fn call(&self, _ctx: &Context) -> Result<H> {
        self.validate()?;
        Ok(H {})
    }
}
