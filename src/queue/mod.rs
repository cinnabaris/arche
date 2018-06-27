pub mod consumer;

#[cfg(feature = "mq-rabbit")]
pub mod rabbitmq;
#[cfg(feature = "mq-rabbit")]
pub use self::rabbitmq::{Config, Producer};

use serde::ser::Serialize;

use super::errors::Result;

pub trait Provider {
    fn push<T: Serialize>(
        &self,
        type_: &String,
        content_type: &String,
        priority: u8,
        payload: &T,
    ) -> Result<()>;
}
