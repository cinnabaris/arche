use std::ops::Deref;

use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use validator::Validate;

use super::super::super::super::{
    errors::Result,
    graphql::{context::Context, H},
    orm::schema::friend_links,
    rfc::UtcDateTime,
};

#[derive(GraphQLObject, Debug, Serialize)]
pub struct FriendLink {
    pub id: String,
    pub title: String,
    pub home: String,
    pub logo: String,
    pub position: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Show {
    #[validate(length(min = "1"))]
    pub id: String,
}

impl Show {
    pub fn call(&self, ctx: &Context) -> Result<FriendLink> {
        self.validate()?;
        let id: i64 = self.id.parse()?;
        let db = ctx.db.deref();
        let (title, home, logo, position, updated_at) = friend_links::dsl::friend_links
            .select((
                friend_links::dsl::title,
                friend_links::dsl::home,
                friend_links::dsl::logo,
                friend_links::dsl::position,
                friend_links::dsl::updated_at,
            ))
            .filter(friend_links::dsl::id.eq(&id))
            .first::<(String, String, String, i16, NaiveDateTime)>(db)?;

        Ok(FriendLink {
            id: self.id.clone(),
            title: title,
            home: home,
            logo: logo,
            position: position as i32,
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
        ctx.admin()?;
        let db = ctx.db.deref();
        let it = friend_links::dsl::friend_links.filter(friend_links::dsl::id.eq(&id));
        delete(it).execute(db)?;
        Ok(H::new())
    }
}

pub fn list(ctx: &Context) -> Result<Vec<FriendLink>> {
    ctx.admin()?;
    let db = ctx.db.deref();
    let items = friend_links::dsl::friend_links
        .select((
            friend_links::dsl::id,
            friend_links::dsl::title,
            friend_links::dsl::home,
            friend_links::dsl::logo,
            friend_links::dsl::position,
            friend_links::dsl::updated_at,
        ))
        .order(friend_links::dsl::position.asc())
        .load::<(i64, String, String, String, i16, NaiveDateTime)>(db)?;

    Ok(items
        .iter()
        .map(|(id, title, home, logo, position, updated_at)| FriendLink {
            id: id.to_string(),
            title: title.clone(),
            home: home.clone(),
            logo: logo.clone(),
            position: *position as i32,
            updated_at: updated_at.to_utc(),
        })
        .collect())
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Create {
    #[validate(length(min = "1"))]
    pub title: String,
    #[validate(length(min = "1"))]
    pub home: String,
    #[validate(length(min = "1"))]
    pub logo: String,
    pub position: i32,
}

impl Create {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let db = ctx.db.deref();
        let now = Utc::now().naive_utc();
        insert_into(friend_links::dsl::friend_links)
            .values((
                friend_links::dsl::title.eq(&self.title),
                friend_links::dsl::home.eq(&self.home),
                friend_links::dsl::logo.eq(&self.logo),
                friend_links::dsl::position.eq(&(self.position as i16)),
                friend_links::dsl::updated_at.eq(&now),
                friend_links::dsl::created_at.eq(&now),
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
    pub title: String,
    #[validate(length(min = "1"))]
    pub home: String,
    #[validate(length(min = "1"))]
    pub logo: String,
    pub position: i32,
}

impl Update {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let id = self.id.parse::<i64>()?;
        let db = ctx.db.deref();
        let now = Utc::now().naive_utc();
        let it = friend_links::dsl::friend_links.filter(friend_links::dsl::id.eq(&id));
        update(it)
            .set((
                friend_links::dsl::title.eq(&self.title),
                friend_links::dsl::home.eq(&self.home),
                friend_links::dsl::logo.eq(&self.logo),
                friend_links::dsl::position.eq(&(self.position as i16)),
                friend_links::dsl::updated_at.eq(&now),
            ))
            .execute(db)?;
        Ok(H::new())
    }
}
