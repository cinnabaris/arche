use std::ops::Deref;

use chrono::Duration;
use r2d2;
use r2d2_redis::RedisConnectionManager;
use redis::cmd;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

use super::super::result::Result;
use super::Cache;

#[derive(Clone)]
pub struct Pool {
    namespace: String,
    pool: r2d2::Pool<RedisConnectionManager>,
}

impl Pool {
    pub fn new(ns: String, pool: r2d2::Pool<RedisConnectionManager>) -> Self {
        Self {
            namespace: ns,
            pool: pool,
        }
    }
    pub fn get(&self) -> Result<Connection> {
        Ok(Connection {
            namespace: self.namespace.clone(),
            con: self.pool.get()?,
        })
    }
}

pub struct Connection {
    namespace: String,
    con: r2d2::PooledConnection<RedisConnectionManager>,
}

impl Cache for Connection {
    fn keys(&self) -> Result<Vec<(String, isize)>> {
        let mut items = Vec::new();
        let k = self.key(&s!("*"));
        let keys: Vec<String> = cmd("keys").arg(k).query(self.con.deref())?;
        for it in keys {
            let ttl: isize = cmd("ttl").arg(it.clone()).query(self.con.deref())?;
            items.push((it, ttl));
        }
        Ok(items)
    }
    fn get(&self, k: &String) -> Result<Vec<u8>> {
        let k = self.key(k);
        let buf: Vec<u8> = cmd("get").arg(&k).query(self.con.deref())?;
        Ok(buf)
    }

    fn set(&self, k: &String, v: &[u8], t: &Duration) -> Result<()> {
        let k = self.key(k);
        let _: String = cmd("set")
            .arg(&k)
            .arg(v)
            .arg("ex")
            .arg(t.num_seconds())
            .query(self.con.deref())?;
        Ok(())
    }

    fn clear(&self) -> Result<isize> {
        let keys: Vec<String> = cmd("keys").arg(self.key(&s!("*"))).query(self.con.deref())?;

        let len = keys.len();
        if len > 0 {
            let cnt: isize = cmd("del").arg(keys.as_slice()).query(self.con.deref())?;
            return Ok(cnt);
        }
        Ok(0)
    }
}

impl Connection {
    fn key(&self, k: &String) -> String {
        format!("{}://{}", self.namespace, k)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.inner().get() {
            Ok(c) => Outcome::Success(c),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
