use chrono::Duration;

use redis::cmd;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json;

use super::result::Result;

pub type Cache = Redis;

pub fn new(url: &String, namespace: String) -> Result<Cache> {
    return Ok(Redis {
        pool: namespace: namespace,
    });
}

//-----------------------------------------------------------------------------

pub trait Provider: Send + Sync {
    fn keys(&self) -> Result<Vec<(String, isize)>>;
    fn get<K: Serialize, V: DeserializeOwned>(&self, k: &K) -> Result<V>;
    fn set<K: Serialize, V: Serialize>(&self, k: &K, v: &V, t: &Duration) -> Result<()>;
    fn clear(&self) -> Result<isize>;
}
