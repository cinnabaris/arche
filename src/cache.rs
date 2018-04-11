use std::ops::Deref;

use chrono::Duration;
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use redis::cmd;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json;

use super::result::Result;

pub type Cache = Redis;

pub fn new(url: &String, namespace: String) -> Result<Cache> {
    return Ok(Redis {
        pool: Pool::new(RedisConnectionManager::new(&url[..])?)?,
        namespace: namespace,
    });
}

pub fn get<V, F>(ch: &Cache, k: &String, t: i64, f: F) -> Result<V>
where
    F: FnOnce() -> Result<V>,
    V: DeserializeOwned + Serialize,
{
    let k = format!("{}://{}", ch.namespace, k);
    let c = ch.pool.get()?;

    let buf: Vec<u8> = cmd("get").arg(&k).query(c.deref())?;
    if !buf.is_empty() {
        let v = serde_json::from_slice(buf.as_slice())?;
        return Ok(v);
    }

    let v = f()?;
    let _: String = cmd("set")
        .arg(&k)
        .arg(serde_json::to_vec(&v)?.as_slice())
        .arg("ex")
        .arg(Duration::days(t).num_seconds())
        .query(c.deref())?;
    Ok(v)
}

//-----------------------------------------------------------------------------

pub trait Provider: Send + Sync {
    fn keys(&self) -> Result<Vec<(String, isize)>>;
    fn get<K: Serialize, V: DeserializeOwned>(&self, k: &K) -> Result<V>;
    fn set<K: Serialize, V: Serialize>(&self, k: &K, v: &V, t: &Duration) -> Result<()>;
    fn clear(&self) -> Result<isize>;
}

pub struct Redis {
    pool: Pool<RedisConnectionManager>,
    namespace: String,
}

impl Redis {
    fn key<K: Serialize>(&self, k: &K) -> Result<String> {
        Ok(format!(
            "{}://{}",
            self.namespace,
            serde_json::to_string(k)?
        ))
    }
}

impl Provider for Redis {
    fn keys(&self) -> Result<Vec<(String, isize)>> {
        let mut items = Vec::new();
        let c = self.pool.get()?;
        let k = self.key(&s!("*"))?;
        let keys: Vec<String> = cmd("keys").arg(k).query(c.deref())?;
        for it in keys {
            let ttl: isize = cmd("ttl").arg(it.clone()).query(c.deref())?;
            items.push((it, ttl));
        }
        return Ok(items);
    }
    fn get<K: Serialize, V: DeserializeOwned>(&self, k: &K) -> Result<V> {
        let c = self.pool.get()?;
        let k = self.key(k)?;
        let buf: Vec<u8> = cmd("get").arg(&k).query(c.deref())?;
        Ok(serde_json::from_slice(buf.as_slice())?)
    }

    fn set<K: Serialize, V: Serialize>(&self, k: &K, v: &V, t: &Duration) -> Result<()> {
        let c = self.pool.get()?;
        let k = self.key(k)?;
        let _: String = cmd("set")
            .arg(&k)
            .arg(serde_json::to_vec(v)?.as_slice())
            .arg("ex")
            .arg(t.num_seconds())
            .query(c.deref())?;
        return Ok(());
    }

    fn clear(&self) -> Result<isize> {
        let c = self.pool.get()?;
        let keys: Vec<String> = cmd("keys").arg(self.key(&s!("*"))?).query(c.deref())?;

        let len = keys.len();
        if len > 0 {
            let cnt: isize = cmd("del").arg(keys.as_slice()).query(c.deref())?;
            return Ok(cnt);
        }
        return Ok(0);
    }
}
