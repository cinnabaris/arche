use std::ops::Deref;

use chrono::Duration;
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use redis::{cmd, ConnectionAddr, ConnectionInfo};
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json;

use super::errors::Result;

const BAD_PROVIDER: &'static str = "bad cache provider";

pub fn get<K, V, F>(ch: &Cache, key: &String, days: i64, fun: F) -> Result<V>
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
        Duration::days(days),
    )?;
    Ok(val)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub namespace: String,
    pub redis: Option<Redis>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Redis {
    pub host: String,
    pub port: u16,
    pub db: i64,
    pub password: Option<String>,
}

impl Config {
    pub fn open(&self) -> Result<Cache> {
        let mut it = Cache {
            namespace: self.namespace.clone(),
            redis: None,
        };
        if let Some(ref cfg) = self.redis {
            it.redis = Some(Pool::new(RedisConnectionManager::new(ConnectionInfo {
                addr: Box::new(ConnectionAddr::Tcp(cfg.host.clone(), cfg.port)),
                db: cfg.db,
                passwd: cfg.password.clone(),
            })?)?);
        }
        Ok(it)
    }
}

pub struct Cache {
    namespace: String,
    redis: Option<Pool<RedisConnectionManager>>,
}

impl Cache {
    fn key(&self, k: &String) -> String {
        format!("{}://{}", self.namespace, k)
    }

    pub fn keys(&self) -> Result<Vec<(String, isize)>> {
        if let Some(ref pool) = self.redis {
            let con = pool.get()?;
            let con = con.deref();
            let mut items = Vec::new();
            let k = self.key(&String::from("*"));
            let keys: Vec<String> = cmd("keys").arg(k).query(con)?;
            for it in keys {
                let ttl: isize = cmd("ttl").arg(it.clone()).query(con)?;
                items.push((it, ttl));
            }
            return Ok(items);
        }
        Err(BAD_PROVIDER.into())
    }

    pub fn get(&self, key: &String) -> Result<Vec<u8>> {
        let key = self.key(key);
        if let Some(ref pool) = self.redis {
            let con = pool.get()?;
            let con = con.deref();
            return Ok(cmd("get").arg(&key).query::<Vec<u8>>(con)?);
        }
        Err(BAD_PROVIDER.into())
    }
    pub fn set(&self, key: &String, val: &[u8], ttl: Duration) -> Result<()> {
        let key = self.key(key);
        if let Some(ref pool) = self.redis {
            let con = pool.get()?;
            let con = con.deref();
            let _: String = cmd("set")
                .arg(&key)
                .arg(val)
                .arg("ex")
                .arg(ttl.num_seconds())
                .query(con)?;
            return Ok(());
        }
        Err(BAD_PROVIDER.into())
    }

    pub fn clear(&self) -> Result<isize> {
        if let Some(ref pool) = self.redis {
            let con = pool.get()?;
            let con = con.deref();
            let keys: Vec<String> = cmd("keys").arg(self.key(&String::from("*"))).query(con)?;

            let len = keys.len();
            if len > 0 {
                let cnt: isize = cmd("del").arg(keys.as_slice()).query(con)?;
                return Ok(cnt);
            }
            return Ok(0);
        }
        Err(BAD_PROVIDER.into())
    }
}
