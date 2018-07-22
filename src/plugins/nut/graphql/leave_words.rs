use std::ops::Deref;

use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*};
use validator::Validate;

use super::super::super::super::{
    errors::Result,
    graphql::{context::Context, H},
    orm::schema::*,
    rfc::UtcDateTime,
};

#[derive(GraphQLObject, Debug, Serialize)]
pub struct LeaveWord {
    pub id: String,
    pub body: String,
    pub media_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Remove {
    #[validate(length(min = "1"))]
    pub id: String,
}

impl Remove {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let id: i64 = self.id.parse()?;
        ctx.admin()?;
        let db = ctx.db.deref();
        let it = leave_words::dsl::leave_words.filter(leave_words::dsl::id.eq(&id));
        delete(it).execute(db)?;
        Ok(H::new())
    }
}

pub fn list(ctx: &Context) -> Result<Vec<LeaveWord>> {
    ctx.admin()?;
    let db = ctx.db.deref();
    let items = leave_words::dsl::leave_words
        .select((
            leave_words::dsl::id,
            leave_words::dsl::body,
            leave_words::dsl::media_type,
            leave_words::dsl::created_at,
        ))
        .order(leave_words::dsl::created_at.desc())
        .load::<(i64, String, String, NaiveDateTime)>(db)?;

    Ok(items
        .iter()
        .map(|(id, msg, ip, ts)| LeaveWord {
            id: id.to_string(),
            body: msg.clone(),
            media_type: ip.clone(),
            created_at: ts.to_utc(),
        })
        .collect())
}

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
