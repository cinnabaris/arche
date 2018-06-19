#[cfg(feature = "mq-rabbit")]
pub mod rabbitmq;

use std::collections::HashMap;
use std::sync::Arc;

use serde::ser::Serialize;
use serde_json;

use super::{
    context::Context, result::{Error, Result},
};

pub trait Consumer: Send + Sync {
    fn run(
        &self,
        ctx: Arc<Context>,
        id: &String,
        content_type: &String,
        payload: &[u8],
    ) -> Result<()>;
}

//-----------------------------------------------------------------------------

pub struct Worker {
    ctx: Arc<Context>,
    consumers: Arc<HashMap<String, Box<Consumer>>>,
}

impl Worker {
    pub fn new(ctx: Arc<Context>, consumers: HashMap<String, Box<Consumer>>) -> Self {
        Self {
            ctx: ctx,
            consumers: Arc::new(consumers),
        }
    }
    pub fn handle(
        &self,
        _type: &String,
        id: &String,
        content_type: &String,
        payload: &[u8],
    ) -> Result<()> {
        let consumers = Arc::clone(&self.consumers);

        match consumers.get(_type) {
            Some(consumer) => consumer.run(self.ctx.clone(), id, content_type, payload),
            None => Err(Error::WithDescription(format!(
                "can't fine consumer for {:?}",
                _type
            ))),
        }
    }
}

//-----------------------------------------------------------------------------

pub trait Queue: Send + Sync {
    fn publish(
        &self,
        _type: &String,
        content_type: &String,
        priority: u8,
        payload: &[u8],
    ) -> Result<()>;
    fn consume(&self, name: String, worker: Arc<Worker>) -> Result<()>;
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
