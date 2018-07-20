use std::ops::Add;

use chrono::{Duration, NaiveDate, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::super::{
    errors::Result,
    orm::{
        schema::{policies, roles},
        Connection as Db,
    },
};
use super::role::Type as RoleType;

pub fn groups(db: &Db, user: &i64) -> Result<Vec<String>> {
    let today = Utc::now().naive_utc().date();
    let mut items = Vec::new();
    for (rid, nbf, exp) in policies::dsl::policies
        .select((
            policies::dsl::role_id,
            policies::dsl::nbf,
            policies::dsl::exp,
        ))
        .filter(policies::dsl::user_id.eq(&user))
        .load::<(i64, NaiveDate, NaiveDate)>(db)?
    {
        if today.ge(&nbf) && today.le(&exp) {
            if let Ok(name) = roles::dsl::roles
                .select(roles::dsl::name)
                .filter(roles::dsl::id.eq(&rid))
                .filter(roles::dsl::resource_type.is_null())
                .filter(roles::dsl::resource_id.is_null())
                .first::<(String)>(db)
            {
                items.push(name);
            }
        }
    }

    Ok(items)
}

pub fn is(db: &Db, user: &i64, role: &RoleType) -> bool {
    can(db, user, role, &None, &None)
}

pub fn can(
    db: &Db,
    user: &i64,
    role: &RoleType,
    resource_type: &Option<String>,
    resource_id: &Option<i64>,
) -> bool {
    let role = format!("{}", role);
    if let Ok(role) = get_role(db, &role, resource_type, resource_id) {
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
    role: &RoleType,
    resource_type: &Option<String>,
    resource_id: &Option<i64>,
) -> Result<()> {
    let role = format!("{}", role);
    if let Ok(role) = get_role(db, &role, resource_type, resource_id) {
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
    role: &RoleType,
    resource_type: &Option<String>,
    resource_id: &Option<i64>,
    ttl: Duration,
) -> Result<i64> {
    let now = Utc::now().naive_utc();
    let nbf = now.date();
    let exp = now.add(ttl).date();
    apply_by_range(db, user, role, resource_type, resource_id, &nbf, &exp)
}

pub fn apply_by_range(
    db: &Db,
    user: &i64,
    role: &RoleType,
    resource_type: &Option<String>,
    resource_id: &Option<i64>,
    nbf: &NaiveDate,
    exp: &NaiveDate,
) -> Result<i64> {
    let role = format!("{}", role);
    let now = Utc::now().naive_utc();

    let role = match get_role(db, &role, resource_type, resource_id) {
        Ok(id) => id,
        Err(_) => {
            let id = insert_into(roles::dsl::roles)
                .values((
                    roles::dsl::name.eq(&role),
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
                    policies::dsl::exp.eq(exp),
                    policies::dsl::nbf.eq(nbf),
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
                    policies::dsl::exp.eq(exp),
                    policies::dsl::nbf.eq(nbf),
                    policies::dsl::updated_at.eq(&now),
                    policies::dsl::created_at.eq(&now),
                ))
                .returning(policies::dsl::id)
                .get_result::<i64>(db)?;
            Ok(id)
        }
    }
}

fn get_role(
    db: &Db,
    name: &String,
    resource_type: &Option<String>,
    resource_id: &Option<i64>,
) -> Result<i64> {
    if &None == resource_type {
        return Ok(roles::dsl::roles
            .select(roles::dsl::id)
            .filter(roles::dsl::name.eq(name))
            .filter(roles::dsl::resource_type.is_null())
            .filter(roles::dsl::resource_id.is_null())
            .first::<i64>(db)?);
    }
    if &None == resource_id {
        return Ok(roles::dsl::roles
            .select(roles::dsl::id)
            .filter(roles::dsl::name.eq(name))
            .filter(roles::dsl::resource_type.eq(resource_type))
            .filter(roles::dsl::resource_id.is_null())
            .first::<i64>(db)?);
    }
    Ok(roles::dsl::roles
        .select(roles::dsl::id)
        .filter(roles::dsl::name.eq(name))
        .filter(roles::dsl::resource_type.eq(resource_type))
        .filter(roles::dsl::resource_id.eq(resource_id))
        .first::<i64>(db)?)
}
