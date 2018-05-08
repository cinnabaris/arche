use std::ops::Deref;

use chrono::Duration;
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use redis::cmd;

use super::super::result::Result;
use super::Provider;

pub struct Redis {
    namespace: String,
    pool: Pool<RedisConnectionManager>,
}

impl Provider for Redis {
    fn keys(&self) -> Result<Vec<(String, isize)>> {
        let mut items = Vec::new();
        let c = self.pool.get()?;
        let k = self.key(&s!("*"));
        let keys: Vec<String> = cmd("keys").arg(k).query(c.deref())?;
        for it in keys {
            let ttl: isize = cmd("ttl").arg(it.clone()).query(c.deref())?;
            items.push((it, ttl));
        }
        Ok(items)
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
        Ok(())
    }

    fn clear(&self) -> Result<isize> {
        let c = self.pool.get()?;
        let keys: Vec<String> = cmd("keys").arg(self.key(&s!("*"))).query(c.deref())?;

        let len = keys.len();
        if len > 0 {
            let cnt: isize = cmd("del").arg(keys.as_slice()).query(c.deref())?;
            return Ok(cnt);
        }
        Ok(0)
    }
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
