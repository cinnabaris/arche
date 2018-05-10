use std::collections::HashMap;
use std::sync::Arc;

#[cfg(feature = "rabbitmq")]
pub mod rabbitmq;
#[cfg(feature = "rabbitmq")]
pub type Client = rabbitmq::RabbitMQ;

use serde::ser::Serialize;
use serde_json;

use super::{
    context::Context,
    result::{Error, Result},
};

pub trait Consumer: Send + Sync {
    fn run(&self, ctx: &Context, id: &String, content_type: &String, payload: &[u8]) -> Result<()>;
}

//-----------------------------------------------------------------------------

#[derive(Clone)]
pub struct Worker {
    ctx: Context,
    consumers: Arc<HashMap<String, Box<Consumer>>>,
}

impl Worker {
    pub fn new(ctx: Context, consumers: HashMap<String, Box<Consumer>>) -> Self {
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
            Some(consumer) => consumer.run(&self.ctx, id, content_type, payload),
            None => Err(Error::WithDescription(format!(
                "can't fine consumer for {:?}",
                _type
            ))),
        }
    }
}

//-----------------------------------------------------------------------------

pub trait Queue {
    fn publish(
        &self,
        _type: &String,
        content_type: &String,
        priority: u8,
        payload: &[u8],
    ) -> Result<()>;
    fn consume(&self, name: String, worker: Worker) -> Result<()>;
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
