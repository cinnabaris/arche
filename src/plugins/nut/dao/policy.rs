use chrono::{NaiveDate, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::super::{
    errors::Result,
    orm::{schema::policies, Connection as Db},
};
use super::super::models::{Policy, Role};

pub fn is(db: &Db, user: &i64, role: &Role) -> bool {
    can(db, user, role, &None)
}

pub fn can(db: &Db, user: &i64, role: &Role, resource: &Option<String>) -> bool {
    let role = format!("{}", role);
    let it = match resource {
        Some(_) => policies::dsl::policies
            .filter(policies::dsl::user_id.eq(user))
            .filter(policies::dsl::role.eq(&role))
            .filter(policies::dsl::resource.eq(resource))
            .first::<Policy>(db),
        None => policies::dsl::policies
            .filter(policies::dsl::user_id.eq(user))
            .filter(policies::dsl::role.eq(&role))
            .filter(policies::dsl::resource.is_null())
            .first::<Policy>(db),
    };
    if let Ok(it) = it {
        return it.enable();
    }
    false
}

pub fn deny(db: &Db, user: &i64, role: &Role, resource: &Option<String>) -> Result<()> {
    let role = format!("{}", role);
    match resource {
        Some(_) => delete(
            policies::dsl::policies
                .filter(policies::dsl::user_id.eq(user))
                .filter(policies::dsl::role.eq(&role))
                .filter(policies::dsl::resource.eq(resource)),
        ).execute(db),
        None => delete(
            policies::dsl::policies
                .filter(policies::dsl::user_id.eq(user))
                .filter(policies::dsl::role.eq(&role))
                .filter(policies::dsl::resource.is_null()),
        ).execute(db),
    }?;
    Ok(())
}

pub fn apply(
    db: &Db,
    user: &i64,
    role: &Role,
    resource: &Option<String>,
    nbf: &NaiveDate,
    exp: &NaiveDate,
) -> Result<i64> {
    let role = format!("{}", role);
    let now = Utc::now().naive_utc();

    let id = match resource {
        Some(_) => policies::dsl::policies
            .select(policies::dsl::id)
            .filter(policies::dsl::user_id.eq(user))
            .filter(policies::dsl::role.eq(&role))
            .filter(policies::dsl::resource.eq(resource))
            .first::<i64>(db),
        None => policies::dsl::policies
            .select(policies::dsl::id)
            .filter(policies::dsl::user_id.eq(user))
            .filter(policies::dsl::role.eq(&role))
            .filter(policies::dsl::resource.is_null())
            .first::<i64>(db),
    };

    match id {
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
                    policies::dsl::role.eq(&role),
                    policies::dsl::resource.eq(resource),
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
