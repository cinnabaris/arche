use chrono::Utc;
use diesel::{insert_into, prelude::*};

use super::super::super::super::{
    errors::Result,
    orm::{schema::logs, Connection as Db},
};

pub fn add(db: &Db, user: &i64, ip: &String, message: &String) -> Result<i64> {
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
