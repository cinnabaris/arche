use chrono::Duration;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use super::result::Result;

pub struct Context<R: Repository, C: Cache, Q: Queue> {
    repository: R,
    cache: C,
    queue: Q,
}

impl<R: Repository, C: Cache, Q: Queue> Context<R, C, Q> {
    pub fn new(r: R, c: C, q: Q) -> Context<R, C, Q> {
        Context {
            repository: r,
            cache: c,
            queue: q,
        }
    }
}

pub trait Repository: Send + Sync {}

pub trait Cache: Send + Sync {
    fn keys(&self) -> Result<Vec<(String, isize)>>;
    fn get<K: Serialize, V: DeserializeOwned>(&self, k: &K) -> Result<V>;
    fn set<K: Serialize, V: Serialize>(&self, k: &K, v: &V, t: &Duration) -> Result<()>;
    fn clear(&self) -> Result<isize>;
}

pub trait Queue: Send + Sync {}
