use std::ops::Deref;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use validator::Validate;

use super::super::super::{graphql::context::Context, result::Result};

#[derive(Serialize, GraphQLInputObject, Deserialize, Debug, Validate)]
#[graphql(description = "Locales by lang")]
pub struct Locales {
    #[validate(length(min = "2", max = "8"))]
    pub lang: String,
}

#[derive(Queryable, GraphQLObject, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LocalesOut {
    pub id: i32,
    pub code: String,
    pub message: String,
    pub updated_at: NaiveDateTime,
}

impl Locales {
    pub fn call(&self, ctx: &Context) -> Result<Vec<LocalesOut>> {
        use super::super::super::schema::locales;
        let db = ctx.db.deref();
        let items = locales::dsl::locales
            .select((
                locales::dsl::id,
                locales::dsl::code,
                locales::dsl::message,
                locales::dsl::updated_at,
            ))
            .filter(locales::dsl::lang.eq(&self.lang))
            .get_results::<LocalesOut>(db)?;
        Ok(items)
    }
}

//-----------------------------------------------------------------------------

#[derive(Serialize, GraphQLInputObject, Deserialize, Debug, Validate)]
#[graphql(description = "Create a new account")]
pub struct UsersSignUp {
    #[validate(length(min = "2", max = "64"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "6", max = "32"))]
    pub password: String,
}

#[derive(Serialize, GraphQLObject, Deserialize, Debug)]
pub struct UsersSignUpOut {
    pub id: String,
    pub name: String,
}

impl UsersSignUp {
    pub fn call(&self, _ctx: &Context) -> Result<UsersSignUpOut> {
        self.validate()?;
        Ok(UsersSignUpOut {
            id: s!("123"),
            name: s!("aaa"),
        })
    }
}
