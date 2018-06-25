pub mod redis;

use chrono::Duration;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json;

use super::errors::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub namespace: String,
    pub redis: Option<redis::Config>,
}

pub enum Cache {
    Redis(redis::Cache),
}

impl Cache {
    pub fn new(cfg: &Config) -> Result<Self> {
        if let Some(ref c) = cfg.redis {
            return Ok(Cache::Redis(redis::Cache::new(
                cfg.namespace.clone(),
                c.open()?,
            )));
        }
        Err("bad cache provider".into())
    }

    pub fn get<K, V, F>(&self, key: &String, days: i64, fun: F) -> Result<V>
    where
        F: FnOnce() -> Result<V>,
        K: Serialize,
        V: DeserializeOwned + Serialize,
    {
        match self {
            Cache::Redis(ch) => {
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
        }
    }
}

pub trait Provider: Send + Sync {
    fn keys(&self) -> Result<Vec<(String, isize)>>;
    fn get(&self, k: &String) -> Result<Vec<u8>>;
    fn set(&self, k: &String, v: &[u8], t: &Duration) -> Result<()>;
    fn clear(&self) -> Result<isize>;
}
