use std::ops::Deref;

use chrono::Duration;
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use redis::cmd;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json;

use super::result::Result;

pub enum Cache {
    Redis((String, Pool<RedisConnectionManager>)),
}

impl Cache {
    pub fn keys(&self) -> Result<Vec<(String, isize)>> {
        match self {
            Cache::Redis((_, pool)) => {
                let mut items = Vec::new();
                let c = pool.get()?;
                let k = self.key(&s!("*"));
                let keys: Vec<String> = cmd("keys").arg(k).query(c.deref())?;
                for it in keys {
                    let ttl: isize = cmd("ttl").arg(it.clone()).query(c.deref())?;
                    items.push((it, ttl));
                }
                Ok(items)
            }
        }
    }
    pub fn get(&self, k: &String) -> Result<Vec<u8>> {
        match self {
            Cache::Redis((_, pool)) => {
                let c = pool.get()?;
                let k = self.key(k);
                let buf: Vec<u8> = cmd("get").arg(&k).query(c.deref())?;
                Ok(buf)
            }
        }
    }

    pub fn set(&self, k: &String, v: &[u8], t: &Duration) -> Result<()> {
        match self {
            Cache::Redis((_, pool)) => {
                let c = pool.get()?;
                let k = self.key(k);
                let _: String = cmd("set")
                    .arg(&k)
                    .arg(v)
                    .arg("ex")
                    .arg(t.num_seconds())
                    .query(c.deref())?;
                Ok(())
            }
        }
    }

    pub fn clear(&self) -> Result<isize> {
        match self {
            Cache::Redis((_, pool)) => {
                let c = pool.get()?;
                let keys: Vec<String> = cmd("keys").arg(self.key(&s!("*"))).query(c.deref())?;

                let len = keys.len();
                if len > 0 {
                    let cnt: isize = cmd("del").arg(keys.as_slice()).query(c.deref())?;
                    return Ok(cnt);
                }
                Ok(0)
            }
        }
    }
    fn key(&self, k: &String) -> String {
        match self {
            Cache::Redis((ns, _)) => format!("{}://{}", ns, k),
        }
    }
}

pub fn get<C, K, V, F>(ch: &Cache, key: &String, days: i64, fun: F) -> Result<V>
where
    F: FnOnce() -> Result<V>,
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
