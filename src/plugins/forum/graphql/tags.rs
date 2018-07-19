use std::ops::Deref;

use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use rocket::http::Status;
use validator::Validate;

use super::super::super::super::{
    errors::{Error, Result},
    graphql::{context::Context, H},
    orm::{
        schema::{forum_tags, forum_topics_tags},
        Connection as Db,
    },
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
pub struct Tag {
    pub id: String,
    pub name: String,
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
        let (name, updated_at) = forum_tags::dsl::forum_tags
            .select((forum_tags::dsl::name, forum_tags::dsl::updated_at))
            .filter(forum_tags::dsl::id.eq(&id))
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
            let it = forum_topics_tags::dsl::forum_topics_tags
                .filter(forum_topics_tags::dsl::tag_id.eq(&id));
            delete(it).execute(db)?;
            let it = forum_tags::dsl::forum_tags.filter(forum_tags::dsl::id.eq(&id));
            delete(it).execute(db)?;
            Ok(())
        })?;
        Ok(H::new())
    }
}

pub fn list(ctx: &Context) -> Result<Vec<Tag>> {
    let db = ctx.db.deref();
    let items = forum_tags::dsl::forum_tags
        .select((
            forum_tags::dsl::id,
            forum_tags::dsl::name,
            forum_tags::dsl::updated_at,
        ))
        .order(forum_tags::dsl::name.asc())
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
        let cnt: i64 = forum_tags::dsl::forum_tags
            .filter(forum_tags::dsl::name.eq(&self.name))
            .count()
            .get_result(db)?;
        if cnt > 0 {
            return Err(Status::BadRequest.reason.into());
        }
        let now = Utc::now().naive_utc();
        insert_into(forum_tags::dsl::forum_tags)
            .values((
                forum_tags::dsl::name.eq(&self.name),
                forum_tags::dsl::updated_at.eq(&now),
                forum_tags::dsl::created_at.eq(&now),
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
        let cnt: i64 = forum_tags::dsl::forum_tags
            .filter(forum_tags::dsl::name.eq(&self.name))
            .count()
            .get_result(db)?;
        if cnt > 0 {
            return Err(Status::BadRequest.reason.into());
        }
        let it = forum_tags::dsl::forum_tags.filter(forum_tags::dsl::id.eq(&id));
        update(it)
            .set((
                forum_tags::dsl::name.eq(&self.name),
                forum_tags::dsl::updated_at.eq(&now),
            ))
            .execute(db)?;
        Ok(H::new())
    }
}
