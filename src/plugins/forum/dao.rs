use super::super::super::{
    orm::Connection as Db,
    plugins::nut::{self, dao::role::Type as RoleType},
};

pub fn is_manager(db: &Db, user: i64) -> bool {
    if nut::dao::policy::is(db, &user, &RoleType::Admin) {
        return true;
    }
    nut::dao::policy::can(
        db,
        &user,
        &RoleType::Manager,
        &Some(super::NAME.to_string()),
        &None,
    )
}
