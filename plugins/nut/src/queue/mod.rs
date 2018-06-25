pub mod consumer;
pub mod rabbitmq;

use serde::ser::Serialize;
use serde_json;

use super::errors::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub name: String,
    pub rabbitmq: Option<rabbitmq::Config>,
}

pub enum Queue {
    RabbitMq(rabbitmq::Queue),
}

impl Queue {
    pub fn new(cfg: &Config) -> Result<Self> {
        if let Some(ref c) = cfg.rabbitmq {
            return Ok(Queue::RabbitMq(rabbitmq::Queue::new(
                cfg.name.clone(),
                c.clone(),
            )));
        }
        Err("bad messing queue provider".into())
    }
}

//-----------------------------------------------------------------------------

pub trait Provider: Send + Sync {
    fn publish(
        &self,
        _type: &String,
        content_type: &String,
        priority: u8,
        payload: &[u8],
    ) -> Result<()>;
}

pub fn put<T: Serialize, Q: Provider>(
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
