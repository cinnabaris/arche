#[cfg(feature = "ch-redis")]
pub mod redis;
#[cfg(feature = "ch-redis")]
pub use self::redis::{Cache, Config};

use serde::{de::DeserializeOwned, ser::Serialize};

use super::errors::Result;

pub trait Provider {
    fn clear(&self) -> Result<isize>;
    fn get<K, V, F>(&self, key: &String, days: i64, fun: F) -> Result<V>
    where
        F: FnOnce() -> Result<V>,
        K: Serialize,
        V: DeserializeOwned + Serialize;
    fn keys(&self) -> Result<Vec<(String, isize)>>;
}
