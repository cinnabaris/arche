use std::net::SocketAddr;
use std::ops::Deref;

use chrono::NaiveDateTime;
use diesel::insert_into;
use diesel::prelude::*;

use serde::ser::Serialize;

use super::super::i18n::Locale;
use super::super::orm::Connection as Db;
use super::super::result::Result;
use super::super::schema::logs;

#[derive(Serialize, Queryable, Deserialize, Debug, Clone)]
pub struct Log {
    pub id: i64,
    #[serde(rename = "userId")]
    pub user_id: i64,
    pub message: String,
    pub ip: String,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
}

impl Log {
    pub fn list(db: &Db, user: &i64) -> Result<Vec<Log>> {
        let it = logs::dsl::logs
            .filter(logs::dsl::user_id.eq(user))
            .get_results::<Log>(db.deref())?;
        Ok(it)
    }
    pub fn add<S: Serialize>(
        db: &Db,
        user: &i64,
        lang: &String,
        remote: &SocketAddr,
        code: &String,
        args: Option<S>,
    ) -> Result<()> {
        let ip = format!("{}", remote.ip());
        let message = Locale::t(db, lang, code, args);

        insert_into(logs::dsl::logs)
            .values((
                logs::dsl::user_id.eq(user),
                logs::dsl::ip.eq(ip),
                logs::dsl::message.eq(&message),
            ))
            .execute(db.deref())?;
        Ok(())
    }
}
