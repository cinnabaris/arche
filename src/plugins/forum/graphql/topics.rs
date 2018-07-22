use std::ops::Deref;

use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use rocket::http::Status;
use validator::Validate;

use super::super::super::super::{
    errors::{Error, Result},
    graphql::{context::Context, H},
    orm::schema::{forum_posts, forum_tags, forum_topics, forum_topics_tags},
    rfc::UtcDateTime,
};
use super::super::dao;
use super::tags::Tag;

#[derive(GraphQLObject, Debug, Serialize)]
pub struct Topic {
    pub id: String,
    pub lang: String,
    pub title: String,
    pub body: String,
    pub media_type: String,
    pub user_id: String,
    pub editable: bool,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<Tag>,
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
        let is_manager = dao::is_manager(db, user.id);

        let tag_ids = forum_topics_tags::dsl::forum_topics_tags
            .select(forum_topics_tags::dsl::tag_id)
            .filter(forum_topics_tags::dsl::topic_id.eq(id))
            .load::<(i64)>(db)?;
        let mut tags = Vec::new();
        for id in tag_ids {
            let (name, updated_at) = forum_tags::dsl::forum_tags
                .select((forum_tags::dsl::name, forum_tags::dsl::updated_at))
                .filter(forum_tags::dsl::id.eq(&id))
                .first::<(String, NaiveDateTime)>(db)?;
            tags.push(Tag {
                id: id.to_string(),
                name: name,
                updated_at: updated_at.to_utc(),
            });
        }

        let (user_id, lang, title, body, media_type, updated_at) = forum_topics::dsl::forum_topics
            .select((
                forum_topics::dsl::user_id,
                forum_topics::dsl::lang,
                forum_topics::dsl::title,
                forum_topics::dsl::body,
                forum_topics::dsl::media_type,
                forum_topics::dsl::updated_at,
            ))
            .filter(forum_topics::dsl::id.eq(&id))
            .first::<(i64, String, String, String, String, NaiveDateTime)>(db)?;

        Ok(Topic {
            id: self.id.clone(),
            user_id: user_id.to_string(),
            lang: lang,
            title: title,
            body: body,
            media_type: media_type,
            editable: is_manager || user_id == user.id,
            tags: tags,
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
            let it = forum_posts::dsl::forum_posts.filter(forum_posts::dsl::topic_id.eq(&id));
            delete(it).execute(db)?;
            let it = forum_topics_tags::dsl::forum_topics_tags
                .filter(forum_topics_tags::dsl::topic_id.eq(&id));
            delete(it).execute(db)?;
            let it = forum_topics::dsl::forum_topics.filter(forum_topics::dsl::id.eq(&id));
            delete(it).execute(db)?;
            Ok(())
        })?;
        Ok(H::new())
    }
}

pub fn list(ctx: &Context) -> Result<Vec<Topic>> {
    let user = ctx.current_user()?;
    let db = ctx.db.deref();
    let is_manager = dao::is_manager(db, user.id);
    let items = forum_topics::dsl::forum_topics
        .select((
            forum_topics::dsl::id,
            forum_topics::dsl::user_id,
            forum_topics::dsl::lang,
            forum_topics::dsl::title,
            forum_topics::dsl::body,
            forum_topics::dsl::media_type,
            forum_topics::dsl::updated_at,
        ))
        .order(forum_topics::dsl::updated_at.desc())
        .load::<(i64, i64, String, String, String, String, NaiveDateTime)>(db)?;

    Ok(items
        .iter()
        .map(
            |(id, user_id, lang, title, body, media_type, updated_at)| Topic {
                id: id.to_string(),
                user_id: user_id.to_string(),
                lang: lang.clone(),
                title: title.clone(),
                body: body.clone(),
                media_type: media_type.clone(),
                editable: is_manager || *user_id == user.id,
                updated_at: updated_at.to_utc(),
                tags: Vec::new(),
            },
        )
        .collect())
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Create {
    #[validate(length(min = "1"))]
    pub title: String,
    #[validate(length(min = "1"))]
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
    pub tags: Vec<String>,
}

impl Create {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let db = ctx.db.deref();
        let user = ctx.current_user()?;
        db.transaction::<_, Error, _>(|| {
            let now = Utc::now().naive_utc();
            let id = insert_into(forum_topics::dsl::forum_topics)
                .values((
                    forum_topics::dsl::user_id.eq(&user.id),
                    forum_topics::dsl::lang.eq(&ctx.locale),
                    forum_topics::dsl::title.eq(&self.title),
                    forum_topics::dsl::body.eq(&self.body),
                    forum_topics::dsl::media_type.eq(&self.media_type),
                    forum_topics::dsl::updated_at.eq(&now),
                    forum_topics::dsl::created_at.eq(&now),
                ))
                .returning(forum_topics::dsl::id)
                .get_result::<i64>(db)?;
            for t in self.tags.iter() {
                insert_into(forum_topics_tags::dsl::forum_topics_tags)
                    .values((
                        forum_topics_tags::dsl::topic_id.eq(&id),
                        forum_topics_tags::dsl::tag_id.eq(&t.parse::<i64>()?),
                    ))
                    .execute(db)?;
            }
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
    pub title: String,
    #[validate(length(min = "1"))]
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
    pub tags: Vec<String>,
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

        db.transaction::<_, Error, _>(|| {
            let now = Utc::now().naive_utc();
            let it = forum_topics::dsl::forum_topics.filter(forum_topics::dsl::id.eq(&id));
            update(it)
                .set((
                    forum_topics::dsl::title.eq(&self.title),
                    forum_topics::dsl::body.eq(&self.body),
                    forum_topics::dsl::media_type.eq(&self.media_type),
                    forum_topics::dsl::updated_at.eq(&now),
                ))
                .execute(db)?;
            let it = forum_topics_tags::dsl::forum_topics_tags
                .filter(forum_topics_tags::dsl::topic_id.eq(&id));
            delete(it).execute(db)?;
            for t in self.tags.iter() {
                insert_into(forum_topics_tags::dsl::forum_topics_tags)
                    .values((
                        forum_topics_tags::dsl::topic_id.eq(&id),
                        forum_topics_tags::dsl::tag_id.eq(&t.parse::<i64>()?),
                    ))
                    .execute(db)?;
            }
            Ok(())
        })?;
        Ok(H::new())
    }
}
