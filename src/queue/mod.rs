#[cfg(feature = "mq-rabbit")]
pub mod rabbitmq;
pub mod worker;

use std::sync::Arc;

use serde::ser::Serialize;
use serde_json;

use super::{context::Context, result::Result};

pub trait Queue: Send + Sync {
    fn publish(
        &self,
        _type: &String,
        content_type: &String,
        priority: u8,
        payload: &[u8],
    ) -> Result<()>;
    fn consume(&self, name: String, worker: Arc<Context>) -> Result<()>;
}

//-----------------------------------------------------------------------------

pub fn put<T: Serialize, Q: Queue>(
    qu: &Q,
    _type: &String,
    content_type: &String,
    priority: u8,
    task: &T,
) -> Result<()> {
    qu.publish(
        _type,
        content_type,
        priority,
        serde_json::to_vec(task)?.as_slice(),
    )
}

//-----------------------------------------------------------------------------
