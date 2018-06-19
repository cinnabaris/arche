#[cfg(feature = "ch-redis")]
pub mod redis;

use chrono::Duration;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json;

use super::result::Result;

pub fn get<C, K, V, F>(ch: &C, key: &String, days: i64, fun: F) -> Result<V>
where
    F: FnOnce() -> Result<V>,
    K: Serialize,
    V: DeserializeOwned + Serialize,
    C: Cache,
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
        &Duration::days(days),
    )?;
    Ok(val)
}

pub trait Cache: Send + Sync {
    fn keys(&self) -> Result<Vec<(String, isize)>>;
    fn get(&self, k: &String) -> Result<Vec<u8>>;
    fn set(&self, k: &String, v: &[u8], t: &Duration) -> Result<()>;
    fn clear(&self) -> Result<isize>;
}
