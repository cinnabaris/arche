use std::ops::Deref;

use chrono::Duration;
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use redis::{cmd, ConnectionAddr, ConnectionInfo};

use super::super::errors::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub db: i64,
    pub password: Option<String>,
}

impl Config {
    pub fn open(&self) -> Result<Pool<RedisConnectionManager>> {
        Ok(Pool::new(RedisConnectionManager::new(ConnectionInfo {
            addr: Box::new(ConnectionAddr::Tcp(self.host.clone(), self.port)),
            db: self.db,
            passwd: self.password.clone(),
        })?)?)
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
    fn get(&self, k: &String) -> Result<Vec<u8>> {
        let con = self.pool.get()?;
        let con = con.deref();
        let k = self.key(k);
        let buf: Vec<u8> = cmd("get").arg(&k).query(con)?;
        Ok(buf)
    }

    fn set(&self, k: &String, v: &[u8], t: &Duration) -> Result<()> {
        let con = self.pool.get()?;
        let con = con.deref();
        let k = self.key(k);
        let _: String = cmd("set")
            .arg(&k)
            .arg(v)
            .arg("ex")
            .arg(t.num_seconds())
            .query(con)?;
        Ok(())
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
