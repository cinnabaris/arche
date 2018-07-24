use super::super::super::{
    orm::Connection as Db,
    plugins::nut::{dao::policy as policy_dao, models::Role},
};

pub fn is_manager(db: &Db, user: i64) -> bool {
    if policy_dao::is(db, &user, &Role::Admin) {
        return true;
    }
    policy_dao::can(db, &user, &Role::Manager, &Some(super::NAME.to_string()))
}
