use std::convert::From;
use std::ops::Deref;

use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use rocket::http::Status;
use validator::Validate;

use super::super::super::super::{
    errors::{Error, Result},
    graphql::{context::Context, H},
    orm::schema::{caring_posts, caring_topics, caring_topics_users},
    rfc::UtcDateTime,
    utils::DATETIME_FORMAT,
};
use super::super::models;

#[derive(GraphQLObject, Debug, Serialize)]
pub struct Post {
    pub id: String,
    pub user_id: String,
    pub topic_id: String,
    pub method: String,
    pub body: String,
    pub media_type: String,
    pub begin: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub editable: bool,
}

impl From<models::Post> for Post {
    fn from(it: models::Post) -> Self {
        Self {
            id: it.id.to_string(),
            user_id: it.user_id.to_string(),
            topic_id: it.topic_id.to_string(),
            method: it.method,
            body: it.body,
            media_type: it.media_type,
            begin: it.begin.to_utc(),
            end: it.end.to_utc(),
            updated_at: it.updated_at.to_utc(),
            editable: false,
        }
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Show {
    #[validate(length(min = "1"))]
    pub id: String,
}

impl Show {
    pub fn call(&self, ctx: &Context) -> Result<Post> {
        self.validate()?;
        let id: i64 = self.id.parse()?;
        let user = ctx.current_user()?;
        let db = ctx.db.deref();

        let it = caring_posts::dsl::caring_posts
            .filter(caring_posts::dsl::id.eq(&id))
            .first::<models::Post>(db)?;

        let editable = user.id == it.user_id;
        let mut ret: Post = it.into();
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

        if caring_posts::dsl::caring_posts
            .select(caring_posts::dsl::user_id)
            .filter(caring_posts::dsl::id.eq(&id))
            .first::<i64>(db)? != user.id
        {
            return Err(Status::Forbidden.reason.into());
        }
        db.transaction::<_, Error, _>(|| {
            let it = caring_posts::dsl::caring_posts.filter(caring_posts::dsl::id.eq(&id));
            delete(it).execute(db)?;
            Ok(())
        })?;
        Ok(H::new())
    }
}

pub fn list(ctx: &Context) -> Result<Vec<Post>> {
    let user = ctx.current_user()?;
    let db = ctx.db.deref();

    // topic owner
    let mut ids = caring_topics::dsl::caring_topics
        .select(caring_topics::dsl::id)
        .filter(caring_topics::dsl::user_id.eq(&user.id))
        .load::<i64>(db)?;
    // topic member
    ids.extend(
        caring_topics_users::dsl::caring_topics_users
            .select(caring_topics_users::dsl::topic_id)
            .filter(caring_topics_users::dsl::user_id.eq(&user.id))
            .load::<i64>(db)?
            .iter(),
    );

    let mut items = Vec::new();
    for id in ids {
        let it = caring_posts::dsl::caring_posts
            .filter(caring_posts::dsl::id.eq(&id))
            .first::<models::Post>(db)?;
        let editable = it.user_id == user.id;
        let mut v: Post = it.into();
        v.editable = editable;
        items.push(v);
    }

    Ok(items)
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Create {
    #[validate(length(min = "1"))]
    pub topic_id: String,
    #[validate(length(min = "1"))]
    pub method: String,
    #[validate(length(min = "1"))]
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
    #[validate(length(min = "1"))]
    pub begin: String,
    #[validate(length(min = "1"))]
    pub end: String,
}

impl Create {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let user = ctx.current_user()?;
        let db = ctx.db.deref();
        let topic = self.topic_id.parse::<i64>()?;

        // only owner of member of topic can create post
        if caring_topics::dsl::caring_topics
            .select(caring_topics::dsl::user_id)
            .filter(caring_topics::dsl::id.eq(&topic))
            .first::<i64>(db)? != user.id
        {
            if let Err(_) = caring_topics_users::dsl::caring_topics_users
                .select(caring_topics_users::dsl::id)
                .filter(caring_topics_users::dsl::user_id.eq(&user.id))
                .filter(caring_topics_users::dsl::topic_id.eq(&topic))
                .first::<i64>(db)
            {
                return Err(Status::Forbidden.reason.into());
            }
        }

        let now = Utc::now().naive_utc();
        insert_into(caring_posts::dsl::caring_posts)
            .values((
                caring_posts::dsl::user_id.eq(&user.id),
                caring_posts::dsl::topic_id.eq(&topic),
                caring_posts::dsl::method.eq(&self.method),
                caring_posts::dsl::body.eq(&self.body),
                caring_posts::dsl::media_type.eq(&self.media_type),
                caring_posts::dsl::begin.eq(&NaiveDateTime::parse_from_str(
                    &self.begin,
                    DATETIME_FORMAT,
                )?),
                caring_posts::dsl::end
                    .eq(&NaiveDateTime::parse_from_str(&self.end, DATETIME_FORMAT)?),
                caring_posts::dsl::updated_at.eq(&now),
                caring_posts::dsl::created_at.eq(&now),
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
    pub method: String,
    #[validate(length(min = "1"))]
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
    #[validate(length(min = "1"))]
    pub begin: String,
    #[validate(length(min = "1"))]
    pub end: String,
}

impl Update {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let id = self.id.parse::<i64>()?;
        let user = ctx.current_user()?;
        let db = ctx.db.deref();

        if caring_posts::dsl::caring_posts
            .select(caring_posts::dsl::user_id)
            .filter(caring_posts::dsl::id.eq(&id))
            .first::<i64>(db)? != user.id
        {
            return Err(Status::Forbidden.reason.into());
        }

        let now = Utc::now().naive_utc();
        let it = caring_posts::dsl::caring_posts.filter(caring_posts::dsl::id.eq(&id));
        update(it)
            .set((
                caring_posts::dsl::body.eq(&self.body),
                caring_posts::dsl::media_type.eq(&self.media_type),
                caring_posts::dsl::updated_at.eq(&now),
            ))
            .execute(db)?;
        Ok(H::new())
    }
}
