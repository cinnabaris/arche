use chrono::Utc;
use diesel::prelude::*;
use diesel::{insert_into, update};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json;

use super::{
    errors::Result,
    orm::{schema::settings, Connection as Db},
    utils::Encryptor,
};

pub fn get<K: Serialize, V: DeserializeOwned>(db: &Db, et: &Encryptor, key: &K) -> Result<V> {
    let key = serde_json::to_string(key)?;
    let (val, salt) = settings::dsl::settings
        .select((settings::dsl::value, settings::dsl::salt))
        .filter(settings::dsl::key.eq(&key))
        .first::<(Vec<u8>, Option<Vec<u8>>)>(db)?;

    let val = match salt {
        Some(salt) => et.decrypt(&val, &salt)?,
        None => val,
    };
    Ok(serde_json::from_slice(val.as_slice())?)
}

pub fn set<K: Serialize, V: Serialize>(
    db: &Db,
    et: &Encryptor,
    k: &K,
    v: &V,
    f: bool,
) -> Result<i64> {
    let key = serde_json::to_string(k)?;
    let buf = serde_json::to_vec(v)?;

    let (val, salt) = if f {
        let (val, salt) = et.encrypt(&buf);
        (val, Some(salt))
    } else {
        (buf, None)
    };

    let now = Utc::now().naive_utc();

    match settings::dsl::settings
        .select(settings::dsl::id)
        .filter(settings::dsl::key.eq(&key))
        .first::<i64>(db)
    {
        Ok(id) => {
            let it = settings::dsl::settings.filter(settings::dsl::id.eq(&id));
            update(it)
                .set((
                    settings::dsl::value.eq(&val),
                    settings::dsl::salt.eq(&salt),
                    settings::dsl::updated_at.eq(&now),
                ))
                .execute(db)?;
            Ok(id)
        }
        Err(_) => Ok(insert_into(settings::dsl::settings)
            .values((
                settings::dsl::key.eq(&key),
                settings::dsl::value.eq(&val),
                settings::dsl::salt.eq(&salt),
                settings::dsl::updated_at.eq(&now),
                settings::dsl::created_at.eq(&now),
            ))
            .returning(settings::dsl::id)
            .get_result::<i64>(db)?),
    }
}
