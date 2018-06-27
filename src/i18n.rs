use diesel::prelude::*;
use mustache;
use serde::ser::Serialize;

use super::{
    errors::{Error, Result}, orm::{schema::locales, Connection as Db},
};

fn tr<S: Serialize>(db: &Db, lang: &String, code: &String, args: &Option<S>) -> Result<String> {
    let msg = locales::dsl::locales
        .select(locales::dsl::message)
        .filter(locales::dsl::lang.eq(lang))
        .filter(locales::dsl::code.eq(code))
        .first::<String>(db)?;
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
