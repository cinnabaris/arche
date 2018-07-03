use chrono::Utc;
use diesel::{insert_into, prelude::*, update};
use mustache;
use serde::ser::Serialize;

use super::{
    errors::{Error, Result},
    orm::{schema::locales, Connection as Db},
};

pub fn get(db: &Db, lang: &String, code: &String) -> Result<String> {
    let msg = locales::dsl::locales
        .select(locales::dsl::message)
        .filter(locales::dsl::lang.eq(lang))
        .filter(locales::dsl::code.eq(code))
        .first::<String>(db)?;
    Ok(msg)
}
pub fn set(db: &Db, lang: &String, code: &String, message: &String) -> Result<i64> {
    let now = Utc::now().naive_utc();
    match locales::dsl::locales
        .select(locales::dsl::id)
        .filter(locales::dsl::lang.eq(lang))
        .filter(locales::dsl::code.eq(code))
        .first::<i64>(db)
    {
        Ok(id) => {
            let it = locales::dsl::locales.filter(locales::dsl::id.eq(&id));
            update(it)
                .set((
                    locales::dsl::message.eq(message),
                    locales::dsl::updated_at.eq(&now),
                ))
                .execute(db)?;
            Ok(id)
        }
        Err(_) => {
            let id = insert_into(locales::dsl::locales)
                .values((
                    locales::dsl::lang.eq(lang),
                    locales::dsl::code.eq(code),
                    locales::dsl::message.eq(message),
                    locales::dsl::updated_at.eq(&now),
                ))
                .returning(locales::dsl::id)
                .get_result::<i64>(db)?;
            Ok(id)
        }
    }
}

fn tr<S: Serialize>(db: &Db, lang: &String, code: &String, args: &Option<S>) -> Result<String> {
    let msg = get(db, lang, code)?;
    if let Some(args) = args {
        let tpl = mustache::compile_str(&msg)?;
        return Ok(tpl.render_to_string(args)?);
    }
    return Ok(msg);
}

pub fn e<S: Serialize>(db: &Db, lang: &String, code: &String, args: &Option<S>) -> Error {
    t(db, lang, code, args).into()
}

pub fn t<S: Serialize>(db: &Db, lang: &String, code: &String, args: &Option<S>) -> String {
    match tr(db, lang, code, args) {
        Ok(msg) => msg,
        Err(e) => {
            error!("{:?}", e);
            format!("{}.{}", lang, code)
        }
    }
}
