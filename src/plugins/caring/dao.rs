use rocket::http::Status;

use super::super::super::{
    errors::Result,
    orm::{schema::*, Connection as Db},
    plugins::nut::{self, dao::role::Type as RoleType},
};

pub fn can_view(db: &Db, user: i64, topic: i64) -> Result<()> {
    if nut::dao::policy::is(db, &user, &RoleType::Admin) {
        return Ok(());
    }
    if nut::dao::policy::can(
        db,
        &user,
        &RoleType::Manager,
        &Some(super::NAME.to_string()),
        &None,
    ) {
        return Ok(());
    }
    if nut::dao::policy::can(
        db,
        &user,
        &RoleType::Member,
        &Some(super::NAME.to_string()),
        &Some(topic),
    ) {
        return Ok(());
    }
    Err(Status::Forbidden.reason.into())
}

pub fn can_edit(db: &Db, user: &i64, topic: &i64) -> Result<()> {
    if nut::dao::policy::is(db, &user, &RoleType::Admin) {
        return Ok(());
    }
    // nut::dao::policy::can(
    //     db,
    //     &user,
    //     &RoleType::Manager,
    //     &Some(super::NAME.to_string()),
    //     &None,
    // )
    Err(Status::Forbidden.reason.into())
}
