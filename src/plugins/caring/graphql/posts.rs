use std::ops::Deref;

use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use rocket::http::Status;
use validator::Validate;

use super::super::super::super::{
    errors::{Error, Result},
    graphql::{context::Context, H},
    orm::schema::forum_posts,
    rfc::UtcDateTime,
};
use super::super::dao;

#[derive(GraphQLObject, Debug, Serialize)]
pub struct Post {
    pub id: String,
    pub user_id: String,
    pub topic_id: String,
    pub post_id: Option<String>,
    pub body: String,
    pub media_type: String,
    pub updated_at: DateTime<Utc>,
    pub editable: bool,
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
        let is_manager = dao::is_manager(db, user.id);

        let (user_id, topic_id, post_id, body, media_type, updated_at) =
            forum_posts::dsl::forum_posts
                .select((
                    forum_posts::dsl::user_id,
                    forum_posts::dsl::topic_id,
                    forum_posts::dsl::post_id,
                    forum_posts::dsl::body,
                    forum_posts::dsl::media_type,
                    forum_posts::dsl::updated_at,
                ))
                .filter(forum_posts::dsl::id.eq(&id))
                .first::<(i64, i64, Option<i64>, String, String, NaiveDateTime)>(db)?;

        Ok(Post {
            id: self.id.clone(),
            user_id: user_id.to_string(),
            topic_id: topic_id.to_string(),
            post_id: match post_id {
                Some(id) => Some(id.to_string()),
                None => None,
            },
            body: body,
            media_type: media_type,
            editable: is_manager || user_id == user.id,
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
        let id: i64 = self.id.parse()?;
        let user = ctx.current_user()?;
        let db = ctx.db.deref();

        if !dao::is_manager(db, user.id) && user.id != id {
            return Err(Status::Forbidden.reason.into());
        }

        db.transaction::<_, Error, _>(|| {
            let it = forum_posts::dsl::forum_posts.filter(forum_posts::dsl::post_id.eq(Some(id)));
            update(it)
                .set(forum_posts::dsl::post_id.eq(&None::<i64>))
                .execute(db)?;
            let it = forum_posts::dsl::forum_posts.filter(forum_posts::dsl::id.eq(&id));
            delete(it).execute(db)?;
            Ok(())
        })?;
        Ok(H::new())
    }
}

pub fn list(ctx: &Context) -> Result<Vec<Post>> {
    let user = ctx.current_user()?;
    let db = ctx.db.deref();
    let is_manager = dao::is_manager(db, user.id);
    let items = forum_posts::dsl::forum_posts
        .select((
            forum_posts::dsl::id,
            forum_posts::dsl::user_id,
            forum_posts::dsl::topic_id,
            forum_posts::dsl::post_id,
            forum_posts::dsl::body,
            forum_posts::dsl::media_type,
            forum_posts::dsl::updated_at,
        ))
        .order(forum_posts::dsl::updated_at.desc())
        .load::<(i64, i64, i64, Option<i64>, String, String, NaiveDateTime)>(db)?;

    Ok(items
        .iter()
        .map(
            |(id, user_id, topic_id, post_id, body, media_type, updated_at)| Post {
                id: id.to_string(),
                user_id: user_id.to_string(),
                topic_id: topic_id.to_string(),
                post_id: match post_id {
                    Some(ref id) => Some(id.to_string()),
                    None => None,
                },
                body: body.clone(),
                media_type: media_type.clone(),
                editable: is_manager || *user_id == user.id,
                updated_at: updated_at.to_utc(),
            },
        )
        .collect())
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Create {
    #[validate(length(min = "1"))]
    pub topic_id: String,
    pub post_id: Option<String>,
    #[validate(length(min = "1"))]
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
}

impl Create {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let user = ctx.current_user()?;
        let db = ctx.db.deref();

        let now = Utc::now().naive_utc();
        let topic_id = self.topic_id.parse::<i64>()?;
        let post_id = match self.post_id {
            Some(ref id) => Some(id.parse::<i64>()?),
            None => None,
        };
        insert_into(forum_posts::dsl::forum_posts)
            .values((
                forum_posts::dsl::user_id.eq(&user.id),
                forum_posts::dsl::topic_id.eq(&topic_id),
                forum_posts::dsl::post_id.eq(&post_id),
                forum_posts::dsl::body.eq(&self.body),
                forum_posts::dsl::media_type.eq(&self.media_type),
                forum_posts::dsl::updated_at.eq(&now),
                forum_posts::dsl::created_at.eq(&now),
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
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
}

impl Update {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let id = self.id.parse::<i64>()?;
        let user = ctx.current_user()?;
        let db = ctx.db.deref();

        if !dao::is_manager(db, user.id) && user.id != id {
            return Err(Status::Forbidden.reason.into());
        }

        let now = Utc::now().naive_utc();
        let it = forum_posts::dsl::forum_posts.filter(forum_posts::dsl::id.eq(&id));
        update(it)
            .set((
                forum_posts::dsl::body.eq(&self.body),
                forum_posts::dsl::media_type.eq(&self.media_type),
                forum_posts::dsl::updated_at.eq(&now),
            ))
            .execute(db)?;
        Ok(H::new())
    }
}
