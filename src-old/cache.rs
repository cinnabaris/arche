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

//-----------------------------------------------------------------------------

pub trait Provider: Send + Sync {
    fn keys(&self) -> Result<Vec<(String, isize)>>;
    fn get<K: Serialize, V: DeserializeOwned>(&self, k: &K) -> Result<V>;
    fn set<K: Serialize, V: Serialize>(&self, k: &K, v: &V, t: &Duration) -> Result<()>;
    fn clear(&self) -> Result<isize>;
}
