use std::convert::From;
use std::ops::Deref;

use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use rocket::http::Status;
use validator::Validate;

use super::super::super::super::{
    errors::{Error, Result},
    graphql::{context::Context, H},
    orm::{
        schema::{caring_posts, caring_topics, members},
        Connection as Db,
    },
    rfc::UtcDateTime,
};
use super::super::super::nut::{dao::policy as policy_dao, models::Role};
use super::super::{dao, models};

pub fn can_view(db: &Db, user: i64, topic: i64) -> Result<()> {
    if dao::is_manager(db, user) {
        return Ok(());
    }
    // TODO
    // if policy_dao::can(
    //     db,
    //     &user,
    //     &Role::Member,
    //     &Some(super::super::NAME.to_string()),
    //     &Some(topic),
    // ) {
    //     return Ok(());
    // }
    Err(Status::Forbidden.reason.into())
}

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
        can_view(db, user.id, id)?;

        let it = caring_topics::dsl::caring_topics
            .filter(caring_topics::dsl::id.eq(&id))
            .first::<models::Topic>(db)?;
        let editable = dao::is_manager(db, user.id) || user.id == it.user_id;
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

        let it = caring_topics::dsl::caring_topics
            .filter(caring_topics::dsl::id.eq(&id))
            .first::<models::Topic>(db)?;
        if !dao::is_manager(db, user.id) && user.id != it.user_id {
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

const NAME: &'static str = "caring.topic";

// pub fn list(ctx: &Context) -> Result<Vec<Topic>> {
//     let user = ctx.current_user()?;
//     let db = ctx.db.deref();
//     let items = if dao::is_manager(db, user.id) {
//         caring_topics::dsl::caring_topics
//             .order(caring_topics::dsl::updated_at.desc())
//             .load::<models::Topic>(db)?
//     } else {
//         let ids = nut::dao::policy::fetch(db, &user.id, &Role::Member, &Name::to_string())?;
//         let mut items = Vec::new();
//         for id in ids {
//             let it = caring_topics::dsl::caring_topics
//                 .filter(caring_topics::dsl::id.eq(&id))
//                 .first::<models::Topic>(db)?;
//             items.push(it);
//         }
//     };
//
//     Ok(items
//         .iter()
//         .map(
//             |(id, user_id, lang, title, body, media_type, updated_at)| Topic {
//                 id: id.to_string(),
//                 user_id: user_id.to_string(),
//                 lang: lang.clone(),
//                 title: title.clone(),
//                 body: body.clone(),
//                 media_type: media_type.clone(),
//                 editable: is_manager || *user_id == user.id,
//                 updated_at: updated_at.to_utc(),
//                 tags: Vec::new(),
//             },
//         )
//         .collect())
// }

// #[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
// pub struct Create {
//     #[validate(length(min = "1"))]
//     pub title: String,
//     #[validate(length(min = "1"))]
//     pub body: String,
//     #[validate(length(min = "1"))]
//     pub media_type: String,
//     pub tags: Vec<String>,
// }
//
// impl Create {
//     pub fn call(&self, ctx: &Context) -> Result<H> {
//         self.validate()?;
//         let db = ctx.db.deref();
//         let user = ctx.current_user()?;
//         db.transaction::<_, Error, _>(|| {
//             let now = Utc::now().naive_utc();
//             let id = insert_into(caring_topics::dsl::caring_topics)
//                 .values((
//                     caring_topics::dsl::user_id.eq(&user.id),
//                     caring_topics::dsl::lang.eq(&ctx.locale),
//                     caring_topics::dsl::title.eq(&self.title),
//                     caring_topics::dsl::body.eq(&self.body),
//                     caring_topics::dsl::media_type.eq(&self.media_type),
//                     caring_topics::dsl::updated_at.eq(&now),
//                     caring_topics::dsl::created_at.eq(&now),
//                 ))
//                 .returning(caring_topics::dsl::id)
//                 .get_result::<i64>(db)?;
//             for t in self.tags.iter() {
//                 insert_into(caring_topics_tags::dsl::caring_topics_tags)
//                     .values((
//                         caring_topics_tags::dsl::topic_id.eq(&id),
//                         caring_topics_tags::dsl::tag_id.eq(&t.parse::<i64>()?),
//                     ))
//                     .execute(db)?;
//             }
//             Ok(())
//         })?;
//
//         Ok(H::new())
//     }
// }
//
// #[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
// pub struct Update {
//     #[validate(length(min = "1"))]
//     pub id: String,
//     #[validate(length(min = "1"))]
//     pub title: String,
//     #[validate(length(min = "1"))]
//     pub body: String,
//     #[validate(length(min = "1"))]
//     pub media_type: String,
//     pub tags: Vec<String>,
// }
//
// impl Update {
//     pub fn call(&self, ctx: &Context) -> Result<H> {
//         self.validate()?;
//         let id = self.id.parse::<i64>()?;
//         let user = ctx.current_user()?;
//         let db = ctx.db.deref();
//
//         if !dao::is_manager(db, user.id) && user.id != id {
//             return Err(Status::Forbidden.reason.into());
//         }
//
//         db.transaction::<_, Error, _>(|| {
//             let now = Utc::now().naive_utc();
//             let it = caring_topics::dsl::caring_topics.filter(caring_topics::dsl::id.eq(&id));
//             update(it)
//                 .set((
//                     caring_topics::dsl::title.eq(&self.title),
//                     caring_topics::dsl::body.eq(&self.body),
//                     caring_topics::dsl::media_type.eq(&self.media_type),
//                     caring_topics::dsl::updated_at.eq(&now),
//                 ))
//                 .execute(db)?;
//             let it = caring_topics_tags::dsl::caring_topics_tags
//                 .filter(caring_topics_tags::dsl::topic_id.eq(&id));
//             delete(it).execute(db)?;
//             for t in self.tags.iter() {
//                 insert_into(caring_topics_tags::dsl::caring_topics_tags)
//                     .values((
//                         caring_topics_tags::dsl::topic_id.eq(&id),
//                         caring_topics_tags::dsl::tag_id.eq(&t.parse::<i64>()?),
//                     ))
//                     .execute(db)?;
//             }
//             Ok(())
//         })?;
//         Ok(H::new())
//     }
// }
