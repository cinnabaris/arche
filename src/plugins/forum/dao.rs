use super::super::super::{
    orm::Connection as Db,
    plugins::nut::{self, dao::role::Type as RoleType},
};
use super::FORUM;

pub fn is_manager(db: &Db, user: i64) -> bool {
    if nut::dao::policy::is(db, &user, &RoleType::Admin) {
        return true;
    }
    nut::dao::policy::is(db, &user, &RoleType::By(FORUM.to_string()))
}
