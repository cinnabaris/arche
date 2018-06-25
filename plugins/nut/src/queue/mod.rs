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

impl Config {
    pub fn open(&self) -> Result<Box<Queue>> {
        if let Some(ref cfg) = self.rabbitmq {
            return Ok(Box::new(rabbitmq::Queue::new(
                self.name.clone(),
                cfg.clone(),
            )));
        }
        Err("bad messaging queue provider".into())
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
}

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
