use std::fmt;
use std::ops::Add;

use chrono::{Duration, NaiveDate, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use uuid::Uuid;

use super::super::super::{
    errors::Result,
    orm::{schema::*, Connection as Db},
    utils,
};
pub fn can(
    db: &Db,
    user: &i64,
    role: &String,
    resource_type: &Option<String>,
    resource_id: &Option<i64>,
) -> bool {
    if let Ok(role) = roles::dsl::roles
        .select(roles::dsl::id)
        .filter(roles::dsl::name.eq(role))
        .filter(roles::dsl::resource_type.eq(resource_type))
        .filter(roles::dsl::resource_id.eq(resource_id))
        .first::<i64>(db)
    {
        if let Ok((nbf, exp)) = policies::dsl::policies
            .select((policies::dsl::nbf, policies::dsl::exp))
            .filter(policies::dsl::user_id.eq(user))
            .filter(policies::dsl::role_id.eq(&role))
            .first::<(NaiveDate, NaiveDate)>(db)
        {
            let today = Utc::now().naive_utc().date();
            return today.ge(&nbf) && today.le(&exp);
        }
    }
    false
}

pub fn deny(
    db: &Db,
    user: &i64,
    role: &String,
    resource_type: &Option<String>,
    resource_id: &Option<i64>,
) -> Result<()> {
    if let Ok(role) = roles::dsl::roles
        .select(roles::dsl::id)
        .filter(roles::dsl::name.eq(role))
        .filter(roles::dsl::resource_type.eq(resource_type))
        .filter(roles::dsl::resource_id.eq(resource_id))
        .first::<i64>(db)
    {
        let it = policies::dsl::policies
            .filter(policies::dsl::user_id.eq(user))
            .filter(policies::dsl::role_id.eq(&role));
        delete(it).execute(db)?;
    }
    Ok(())
}

pub fn apply(
    db: &Db,
    user: &i64,
    role: &String,
    resource_type: &Option<String>,
    resource_id: &Option<i64>,
    ttl: Duration,
) -> Result<i64> {
    let now = Utc::now().naive_utc();
    let nbf = now.date();
    let exp = now.add(ttl).date();

    let role = match roles::dsl::roles
        .select(roles::dsl::id)
        .filter(roles::dsl::name.eq(role))
        .filter(roles::dsl::resource_type.eq(resource_type))
        .filter(roles::dsl::resource_id.eq(resource_id))
        .first::<i64>(db)
    {
        Ok(id) => id,
        Err(_) => {
            let id = insert_into(roles::dsl::roles)
                .values((
                    roles::dsl::name.eq(role),
                    roles::dsl::resource_type.eq(resource_type),
                    roles::dsl::resource_id.eq(resource_id),
                    roles::dsl::created_at.eq(&now),
                ))
                .returning(roles::dsl::id)
                .get_result::<i64>(db)?;
            id
        }
    };

    match policies::dsl::policies
        .select(policies::dsl::id)
        .filter(policies::dsl::user_id.eq(user))
        .filter(policies::dsl::role_id.eq(&role))
        .first::<i64>(db)
    {
        Ok(id) => {
            let it = policies::dsl::policies.filter(policies::dsl::id.eq(&id));
            update(it)
                .set((
                    policies::dsl::exp.eq(&exp),
                    policies::dsl::nbf.eq(&nbf),
                    policies::dsl::updated_at.eq(&now),
                ))
                .execute(db)?;
            Ok(id)
        }
        Err(_) => {
            let id = insert_into(policies::dsl::policies)
                .values((
                    policies::dsl::user_id.eq(user),
                    policies::dsl::role_id.eq(&role),
                    policies::dsl::exp.eq(&exp),
                    policies::dsl::nbf.eq(&nbf),
                    policies::dsl::updated_at.eq(&now),
                    policies::dsl::created_at.eq(&now),
                ))
                .returning(policies::dsl::id)
                .get_result::<i64>(db)?;
            Ok(id)
        }
    }
}

pub fn add_log(db: &Db, user: &i64, ip: &String, message: &String) -> Result<i64> {
    let now = Utc::now().naive_utc();
    let id = insert_into(logs::dsl::logs)
        .values((
            logs::dsl::user_id.eq(user),
            logs::dsl::ip.eq(ip),
            logs::dsl::message.eq(message),
            logs::dsl::created_at.eq(&now),
        ))
        .returning(logs::dsl::id)
        .get_result::<i64>(db)?;
    Ok(id)
}

pub fn add_user_by_email(db: &Db, name: &String, email: &String, password: &String) -> Result<i64> {
    let now = Utc::now().naive_utc();
    match users::dsl::users
        .select(users::dsl::id)
        .filter(users::dsl::email.eq(email))
        .first::<i64>(db)
    {
        Ok(_) => Err("email already exist".into()),
        Err(_) => {
            let password = utils::hash::sum(password.as_bytes())?;
            let id = insert_into(users::dsl::users)
                .values((
                    users::dsl::email.eq(email),
                    users::dsl::name.eq(name),
                    users::dsl::password.eq(&Some(password)),
                    users::dsl::provider_type.eq(&format!("{}", UserType::Email)),
                    users::dsl::provider_id.eq(email),
                    users::dsl::uid.eq(&Uuid::new_v4().to_string()),
                    users::dsl::sign_in_count.eq(0),
                    users::dsl::updated_at.eq(&now),
                    users::dsl::created_at.eq(&now),
                ))
                .returning(users::dsl::id)
                .get_result::<i64>(db)?;
            Ok(id)
        }
    }
}

#[derive(Debug)]
pub enum Role {
    Admin,
    Root,
    Member,
    By(String),
}

impl fmt::Display for Role {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::Admin => fmt.write_str("admin"),
            Role::Root => fmt.write_str("root"),
            Role::Member => fmt.write_str("member"),
            Role::By(n) => fmt.write_str(&n),
        }
    }
}

#[derive(Debug)]
pub enum UserType {
    Google,
    Facebook,
    Line,
    Github,
    WeChat,
    Email,
}

impl fmt::Display for UserType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserType::Google => fmt.write_str("google"),
            UserType::Facebook => fmt.write_str("facebook"),
            UserType::Github => fmt.write_str("github"),
            UserType::WeChat => fmt.write_str("wechat"),
            UserType::Line => fmt.write_str("line"),
            UserType::Email => fmt.write_str("email"),
        }
    }
}
