use std::ops::Deref;

use chrono::Duration;
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use redis::{cmd, ConnectionAddr, ConnectionInfo};
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json;

use super::super::errors::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub db: i64,
    pub password: Option<String>,
    pub namespace: String,
}

impl Config {
    pub fn open(&self) -> Result<Cache> {
        Ok(Cache {
            namespace: self.namespace.clone(),
            pool: Pool::new(RedisConnectionManager::new(ConnectionInfo {
                addr: Box::new(ConnectionAddr::Tcp(self.host.clone(), self.port)),
                db: self.db,
                passwd: self.password.clone(),
            })?)?,
        })
    }
}

pub struct Cache {
    namespace: String,
    pool: Pool<RedisConnectionManager>,
}

impl Cache {
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

impl super::Provider for Cache {
    fn keys(&self) -> Result<Vec<(String, isize)>> {
        let con = self.pool.get()?;
        let con = con.deref();
        let mut items = Vec::new();
        let k = self.key(&String::from("*"));
        let keys: Vec<String> = cmd("keys").arg(k).query(con)?;
        for it in keys {
            let ttl: isize = cmd("ttl").arg(it.clone()).query(con)?;
            items.push((it, ttl));
        }
        Ok(items)
    }

    fn get<K, V, F>(&self, key: &String, days: i64, fun: F) -> Result<V>
    where
        F: FnOnce() -> Result<V>,
        K: Serialize,
        V: DeserializeOwned + Serialize,
    {
        let con = self.pool.get()?;
        let con = con.deref();
        let key = self.key(key);

        if let Ok(buf) = cmd("get").arg(&key).query::<Vec<u8>>(con) {
            if let Ok(val) = serde_json::from_slice(buf.as_slice()) {
                return Ok(val);
            }
        }
        let val = fun()?;
        let _: String = cmd("set")
            .arg(&key)
            .arg(serde_json::to_vec(&val)?.as_slice())
            .arg("ex")
            .arg(Duration::days(days).num_seconds())
            .query(con)?;
        Ok(val)
    }

    fn clear(&self) -> Result<isize> {
        let con = self.pool.get()?;
        let con = con.deref();
        let keys: Vec<String> = cmd("keys").arg(self.key(&String::from("*"))).query(con)?;

        let len = keys.len();
        if len > 0 {
            let cnt: isize = cmd("del").arg(keys.as_slice()).query(con)?;
            return Ok(cnt);
        }
        Ok(0)
    }
}
