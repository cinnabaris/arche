use std::ops::Deref;

use chrono::Utc;
use diesel::{insert_into, prelude::*};
use validator::Validate;

use super::super::super::super::{
    errors::Result,
    graphql::{context::Context, H},
    orm::schema::*,
};

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Create {
    #[validate(length(min = "1"))]
    pub media_type: String,
    #[validate(length(min = "1"))]
    pub body: String,
}

impl Create {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let db = ctx.db.deref();
        let now = Utc::now().naive_utc();
        insert_into(leave_words::dsl::leave_words)
            .values((
                leave_words::dsl::media_type.eq(&self.media_type),
                leave_words::dsl::body.eq(&self.body),
                leave_words::dsl::created_at.eq(&now),
            ))
            .execute(db)?;
        Ok(H::new())
    }
}
