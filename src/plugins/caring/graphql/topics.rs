use std::convert::From;
use std::ops::Deref;

use chrono::{DateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use rocket::http::Status;
use validator::Validate;

use super::super::super::super::{
    errors::{Error, Result},
    graphql::{context::Context, H},
    orm::schema::{caring_posts, caring_topics, caring_topics_users},
    rfc::UtcDateTime,
};
use super::super::{dao, models};

#[derive(GraphQLObject, Debug, Serialize)]
pub struct Topic {
    pub id: String,
    pub user_id: String,
    pub tag: String,
    pub name: String,
    pub gender: String,
    pub age: i32,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub reason: String,
    pub media_type: String,
    pub status: String,
    pub editable: bool,
    pub updated_at: DateTime<Utc>,
}

impl From<models::Topic> for Topic {
    fn from(it: models::Topic) -> Self {
        Self {
            id: it.id.to_string(),
            user_id: it.user_id.to_string(),
            editable: false,
            tag: it.tag,
            age: it.age.into(),
            name: it.name,
            phone: it.phone,
            email: it.email,
            gender: it.gender,
            address: it.address,
            reason: it.reason,
            media_type: it.media_type,
            status: it.status,
            updated_at: it.updated_at.to_utc(),
        }
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Show {
    #[validate(length(min = "1"))]
    pub id: String,
}

impl Show {
    pub fn call(&self, ctx: &Context) -> Result<Topic> {
        self.validate()?;
        let id: i64 = self.id.parse()?;
        let user = ctx.current_user()?;
        let db = ctx.db.deref();

        let it = caring_topics::dsl::caring_topics
            .filter(caring_topics::dsl::id.eq(&id))
            .first::<models::Topic>(db)?;
        // only owner of member of topic can view
        if it.user_id != user.id {
            if let Err(_) = caring_topics_users::dsl::caring_topics_users
                .select(caring_topics_users::dsl::id)
                .filter(caring_topics_users::dsl::user_id.eq(&user.id))
                .filter(caring_topics_users::dsl::topic_id.eq(&id))
                .first::<i64>(db)
            {
                return Err(Status::Forbidden.reason.into());
            }
        }
        let editable = user.id == it.user_id;
        let mut ret: Topic = it.into();
        ret.editable = editable;
        Ok(ret)
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
        let id: i64 = self.id.parse()?;
        let user = ctx.current_user()?;
        let db = ctx.db.deref();

        // only owner can remove
        if caring_topics::dsl::caring_topics
            .select(caring_topics::dsl::user_id)
            .filter(caring_topics::dsl::id.eq(&id))
            .first::<i64>(db)? != user.id
        {
            return Err(Status::Forbidden.reason.into());
        }

        db.transaction::<_, Error, _>(|| {
            let it = caring_posts::dsl::caring_posts.filter(caring_posts::dsl::topic_id.eq(&id));
            delete(it).execute(db)?;
            let it = caring_topics::dsl::caring_topics.filter(caring_topics::dsl::id.eq(&id));
            delete(it).execute(db)?;
            Ok(())
        })?;
        Ok(H::new())
    }
}

pub fn list(ctx: &Context) -> Result<Vec<Topic>> {
    let user = ctx.current_user()?;
    let db = ctx.db.deref();
    let mut items = Vec::new();
    // owner
    for it in caring_topics::dsl::caring_topics
        .filter(caring_topics::dsl::user_id.eq(&user.id))
        .load::<models::Topic>(db)?
    {
        let mut v: Topic = it.into();
        v.editable = true;
        items.push(v);
    }
    // member
    for id in caring_topics_users::dsl::caring_topics_users
        .select(caring_topics_users::dsl::topic_id)
        .filter(caring_topics_users::dsl::user_id.eq(&user.id))
        .load::<i64>(db)?
    {
        let it = caring_topics::dsl::caring_topics
            .filter(caring_topics::dsl::id.eq(&id))
            .first::<models::Topic>(db)?;
        let mut v: Topic = it.into();
        v.editable = false;
        items.push(v);
    }

    Ok(items)
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Create {
    #[validate(length(min = "1"))]
    pub member_id: String,
    #[validate(length(min = "1"))]
    pub tag: String,
    #[validate(length(min = "1"))]
    pub name: String,
    #[validate(length(min = "1"))]
    pub gender: String,
    pub age: i32,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    #[validate(length(min = "1"))]
    pub reason: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
    #[validate(length(min = "1"))]
    pub status: String,
}

impl Create {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let db = ctx.db.deref();
        let user = ctx.current_user()?;
        // only manager can create
        if !dao::is_manager(db, user.id) {
            return Err(Status::Forbidden.reason.into());
        }

        let mid = self.member_id.parse::<i64>()?;
        let age = self.age as i16;

        db.transaction::<_, Error, _>(|| {
            let now = Utc::now().naive_utc();
            insert_into(caring_topics::dsl::caring_topics)
                .values((
                    caring_topics::dsl::member_id.eq(&mid),
                    caring_topics::dsl::user_id.eq(&user.id),
                    caring_topics::dsl::tag.eq(&self.tag),
                    caring_topics::dsl::name.eq(&self.name),
                    caring_topics::dsl::gender.eq(&self.gender),
                    caring_topics::dsl::age.eq(&age),
                    caring_topics::dsl::phone.eq(&self.phone),
                    caring_topics::dsl::email.eq(&self.email),
                    caring_topics::dsl::address.eq(&self.address),
                    caring_topics::dsl::reason.eq(&self.reason),
                    caring_topics::dsl::media_type.eq(&self.media_type),
                    caring_topics::dsl::status.eq(&self.status),
                    caring_topics::dsl::updated_at.eq(&now),
                    caring_topics::dsl::created_at.eq(&now),
                ))
                .returning(caring_topics::dsl::id)
                .get_result::<i64>(db)?;
            Ok(())
        })?;

        Ok(H::new())
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Update {
    #[validate(length(min = "1"))]
    pub id: String,
    #[validate(length(min = "1"))]
    pub tag: String,
    #[validate(length(min = "1"))]
    pub name: String,
    #[validate(length(min = "1"))]
    pub gender: String,
    pub age: i32,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    #[validate(length(min = "1"))]
    pub reason: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
    #[validate(length(min = "1"))]
    pub status: String,
}

impl Update {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let id = self.id.parse::<i64>()?;
        let user = ctx.current_user()?;
        let db = ctx.db.deref();

        let age = self.age as i16;
        // only owner
        if caring_topics::dsl::caring_topics
            .select(caring_topics::dsl::user_id)
            .filter(caring_topics::dsl::id.eq(&id))
            .first::<i64>(db)? != user.id
        {
            return Err(Status::Forbidden.reason.into());
        }

        db.transaction::<_, Error, _>(|| {
            let now = Utc::now().naive_utc();
            let it = caring_topics::dsl::caring_topics.filter(caring_topics::dsl::id.eq(&id));
            update(it)
                .set((
                    caring_topics::dsl::tag.eq(&self.tag),
                    caring_topics::dsl::name.eq(&self.name),
                    caring_topics::dsl::gender.eq(&self.gender),
                    caring_topics::dsl::age.eq(&age),
                    caring_topics::dsl::phone.eq(&self.phone),
                    caring_topics::dsl::email.eq(&self.email),
                    caring_topics::dsl::address.eq(&self.address),
                    caring_topics::dsl::reason.eq(&self.reason),
                    caring_topics::dsl::media_type.eq(&self.media_type),
                    caring_topics::dsl::status.eq(&self.status),
                    caring_topics::dsl::updated_at.eq(&now),
                    caring_topics::dsl::created_at.eq(&now),
                ))
                .execute(db)?;
            Ok(())
        })?;
        Ok(H::new())
    }
}
