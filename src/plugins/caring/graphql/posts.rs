use std::ops::Deref;

use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use rocket::http::Status;
use validator::Validate;

use super::super::super::super::{
    errors::{Error, Result},
    graphql::{context::Context, H},
    orm::{schema::caring_posts, Connection as Db},
    rfc::Utc as ToUtc,
};
use super::super::dao;

fn can(db: &Db, user: i64) -> Result<()> {
    if dao::is_manager(db, user) {
        return Ok(());
    }
    Err(Status::Forbidden.reason.into())
}

#[derive(GraphQLObject, Debug, Serialize)]
pub struct Post {
    pub id: String,
    pub topic_id: String,
    pub user_id: String,
    pub method: String,
    pub body: String,
    pub media_type: String,
    pub begin: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Show {
    #[validate(length(min = "1"))]
    pub id: String,
}

impl Show {
    pub fn call(&self, ctx: &Context) -> Result<Tag> {
        self.validate()?;
        let id: i64 = self.id.parse()?;
        let db = ctx.db.deref();
        let (name, updated_at) = caring_tags::dsl::caring_tags
            .select((caring_tags::dsl::name, caring_tags::dsl::updated_at))
            .filter(caring_tags::dsl::id.eq(&id))
            .first::<(String, NaiveDateTime)>(db)?;

        Ok(Tag {
            id: self.id.clone(),
            name: name,
            updated_at: updated_at.to_utc(),
        })
    }
}
#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Remove {
    #[validate(length(min = "1"))]
    pub id: String,
}

impl Remove {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let db = ctx.db.deref();
        let user = ctx.current_user()?;
        can(db, user.id)?;
        let id: i64 = self.id.parse()?;
        db.transaction::<_, Error, _>(|| {
            let it = caring_topics_tags::dsl::caring_topics_tags
                .filter(caring_topics_tags::dsl::tag_id.eq(&id));
            delete(it).execute(db)?;
            let it = caring_tags::dsl::caring_tags.filter(caring_tags::dsl::id.eq(&id));
            delete(it).execute(db)?;
            Ok(())
        })?;
        Ok(H::new())
    }
}

pub fn list(ctx: &Context) -> Result<Vec<Tag>> {
    let db = ctx.db.deref();
    let items = caring_tags::dsl::caring_tags
        .select((
            caring_tags::dsl::id,
            caring_tags::dsl::name,
            caring_tags::dsl::updated_at,
        ))
        .order(caring_tags::dsl::name.asc())
        .load::<(i64, String, NaiveDateTime)>(db)?;

    Ok(items
        .iter()
        .map(|(id, name, updated_at)| Tag {
            id: id.to_string(),
            name: name.clone(),
            updated_at: updated_at.to_utc(),
        })
        .collect())
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Create {
    #[validate(length(min = "1"))]
    pub name: String,
}

impl Create {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let db = ctx.db.deref();
        let user = ctx.current_user()?;
        can(db, user.id)?;
        let cnt: i64 = caring_tags::dsl::caring_tags
            .filter(caring_tags::dsl::name.eq(&self.name))
            .count()
            .get_result(db)?;
        if cnt > 0 {
            return Err(Status::BadRequest.reason.into());
        }
        let now = Utc::now().naive_utc();
        insert_into(caring_tags::dsl::caring_tags)
            .values((
                caring_tags::dsl::name.eq(&self.name),
                caring_tags::dsl::updated_at.eq(&now),
                caring_tags::dsl::created_at.eq(&now),
            ))
            .execute(db)?;
        Ok(H::new())
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Update {
    #[validate(length(min = "1"))]
    pub id: String,
    #[validate(length(min = "1"))]
    pub name: String,
}

impl Update {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let id = self.id.parse::<i64>()?;
        let db = ctx.db.deref();
        let user = ctx.current_user()?;
        can(db, user.id)?;
        let now = Utc::now().naive_utc();
        let cnt: i64 = caring_tags::dsl::caring_tags
            .filter(caring_tags::dsl::name.eq(&self.name))
            .count()
            .get_result(db)?;
        if cnt > 0 {
            return Err(Status::BadRequest.reason.into());
        }
        let it = caring_tags::dsl::caring_tags.filter(caring_tags::dsl::id.eq(&id));
        update(it)
            .set((
                caring_tags::dsl::name.eq(&self.name),
                caring_tags::dsl::updated_at.eq(&now),
            ))
            .execute(db)?;
        Ok(H::new())
    }
}
