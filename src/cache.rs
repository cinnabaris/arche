use std::ops::Deref;

use chrono::Duration;
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use redis::cmd;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json;

use super::result::Result;

pub fn get<C, K, V, F>(ch: &Cache, key: &String, days: i64, fun: F) -> Result<V>
where
    F: FnOnce() -> Result<V>,
    C: Cache,
    K: Serialize,
    V: DeserializeOwned + Serialize,
{
    if let Ok(buf) = ch.get(key) {
        if let Ok(val) = serde_json::from_slice(buf.as_slice()) {
            return Ok(val);
        }
    }

    let val = fun()?;
    ch.set(
        key,
        serde_json::to_vec(&val)?.as_slice(),
        &Duration::days(days),
    )?;
    Ok(val)
}

//-----------------------------------------------------------------------------

pub trait Cache: Send + Sync {
    fn keys(&self) -> Result<Vec<(String, isize)>>;
    fn get(&self, k: &String) -> Result<Vec<u8>>;
    fn set(&self, k: &String, v: &[u8], t: &Duration) -> Result<()>;
    fn clear(&self) -> Result<isize>;
}

//-----------------------------------------------------------------------------

pub struct Redis {
    pool: Pool<RedisConnectionManager>,
    namespace: String,
}

impl Redis {
    pub fn new(ns: String, pool: Pool<RedisConnectionManager>) -> Self {
        Self {
            namespace: ns,
            pool: pool,
        }
    }

    fn key(&self, k: &String) -> String {
        format!("{}://{}", self.namespace, k)
    }
}

impl Cache for Redis {
    fn keys(&self) -> Result<Vec<(String, isize)>> {
        let mut items = Vec::new();
        let c = self.pool.get()?;
        let k = self.key(&s!("*"));
        let keys: Vec<String> = cmd("keys").arg(k).query(c.deref())?;
        for it in keys {
            let ttl: isize = cmd("ttl").arg(it.clone()).query(c.deref())?;
            items.push((it, ttl));
        }
        return Ok(items);
    }
    fn get(&self, k: &String) -> Result<Vec<u8>> {
        let c = self.pool.get()?;
        let k = self.key(k);
        let buf: Vec<u8> = cmd("get").arg(&k).query(c.deref())?;
        Ok(buf)
    }

    fn set(&self, k: &String, v: &[u8], t: &Duration) -> Result<()> {
        let c = self.pool.get()?;
        let k = self.key(k);
        let _: String = cmd("set")
            .arg(&k)
            .arg(v)
            .arg("ex")
            .arg(t.num_seconds())
            .query(c.deref())?;
        return Ok(());
    }

    fn clear(&self) -> Result<isize> {
        let c = self.pool.get()?;
        let keys: Vec<String> = cmd("keys").arg(self.key(&s!("*"))).query(c.deref())?;

        let len = keys.len();
        if len > 0 {
            let cnt: isize = cmd("del").arg(keys.as_slice()).query(c.deref())?;
            return Ok(cnt);
        }
        return Ok(0);
    }
}
