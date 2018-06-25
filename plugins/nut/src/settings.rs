use chrono::Utc;
use diesel::prelude::*;
use diesel::{insert_into, update};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json;

use super::{
    errors::Result,
    orm::{self, schema::settings},
    security::Provider as Encryptor,
};

pub trait Dao {
    fn get(&self, key: &String) -> Result<(Vec<u8>, Option<Vec<u8>>)>;
    fn set(&self, key: &String, val: &Vec<u8>, salt: &Option<Vec<u8>>) -> Result<i64>;
}

impl<'a> Dao for orm::Dao<'a> {
    fn get(&self, key: &String) -> Result<(Vec<u8>, Option<Vec<u8>>)> {
        Ok(settings::dsl::settings
            .select((settings::dsl::value, settings::dsl::salt))
            .filter(settings::dsl::key.eq(&key))
            .first::<(Vec<u8>, Option<Vec<u8>>)>(self.db)?)
    }

    fn set(&self, key: &String, val: &Vec<u8>, salt: &Option<Vec<u8>>) -> Result<i64> {
        let now = Utc::now().naive_utc();

        match settings::dsl::settings
            .select(settings::dsl::id)
            .filter(settings::dsl::key.eq(key))
            .first::<i64>(self.db)
        {
            Ok(id) => {
                let it = settings::dsl::settings.filter(settings::dsl::id.eq(&id));
                update(it)
                    .set((
                        settings::dsl::value.eq(val),
                        settings::dsl::salt.eq(salt),
                        settings::dsl::updated_at.eq(&now),
                    ))
                    .execute(self.db)?;
                Ok(id)
            }
            Err(_) => Ok(insert_into(settings::dsl::settings)
                .values((
                    settings::dsl::key.eq(key),
                    settings::dsl::value.eq(val),
                    settings::dsl::salt.eq(salt),
                    settings::dsl::updated_at.eq(&now),
                ))
                .returning(settings::dsl::id)
                .get_result::<i64>(self.db)?),
        }
    }
}

pub fn get<K: Serialize, V: DeserializeOwned, D: Dao, E: Encryptor>(
    db: &D,
    se: &E,
    k: &K,
) -> Result<V> {
    let k = serde_json::to_string(k)?;
    let (val, salt) = db.get(&k)?;

    let val = match salt {
        Some(salt) => se.decrypt(&val, &salt)?,
        None => val,
    };
    Ok(serde_json::from_slice(val.as_slice())?)
}

pub fn set<K: Serialize, V: Serialize, D: Dao, E: Encryptor>(
    db: &Dao,
    se: &E,
    k: &K,
    v: &V,
    f: bool,
) -> Result<i64> {
    let k = serde_json::to_string(k)?;
    let v = serde_json::to_vec(v)?;

    if f {
        let (val, salt) = se.encrypt(&v);
        db.set(&k, &val, &Some(salt))
    } else {
        db.set(&k, &v, &None)
    }
}
