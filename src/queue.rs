use std::time::{SystemTime, UNIX_EPOCH};

use amqp::{self, Basic};
use log;
use mime;
use serde::ser::Serialize;
use serde_json;
use uuid::Uuid;

use super::{context::Context, errors::Result};

pub const BAD_PROVIDER: &'static str = "bad messing queue provider";

pub fn push<T: Serialize>(
    producer: &Producer,
    type_: &'static str,
    priority: u8,
    payload: &T,
) -> Result<()> {
    producer.push(
        &Uuid::new_v4().to_string(),
        &type_.to_string(),
        &format!("{}", mime::APPLICATION_JSON),
        priority,
        serde_json::to_vec(payload)?.as_slice(),
    )
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub name: String,
    pub rabbitmq: Option<RabbitMQ>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RabbitMQ {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    #[serde(rename = "virtual")]
    pub virtual_: String,
}

impl RabbitMQ {
    fn options(&self) -> amqp::Options {
        amqp::Options {
            host: self.host.clone(),
            port: self.port,
            login: self.user.clone(),
            password: self.password.clone(),
            vhost: self.virtual_.clone(),
            ..Default::default()
        }
    }

    pub fn open<F>(&self, queue: String, f: F) -> Result<()>
    where
        F: Fn(&mut amqp::Channel, &String) -> Result<()>,
    {
        let mut ss = amqp::Session::new(self.options())?;
        let mut ch = ss.open_channel(1)?;
        ch.queue_declare(
            &queue[..],
            false, // passive,
            true,  // durable
            false, // exclusive
            false, // auto_delete
            false, // nowait
            amqp::Table::new(),
        )?;

        f(&mut ch, &queue)?;

        ch.close(200, "Bye")?;
        ss.close(200, "Good Bye");

        Ok(())
    }
}

pub struct Producer {
    cfg: Config,
}

impl Producer {
    pub fn new(cfg: Config) -> Self {
        Self { cfg: cfg }
    }

    pub fn push(
        &self,
        id: &String,
        type_: &String,
        content_type: &String,
        priority: u8,
        payload: &[u8],
    ) -> Result<()> {
        log::info!("push task into queue {}@{}", id, self.cfg.name);
        if let Some(ref cfg) = self.cfg.rabbitmq {
            return cfg.open(self.cfg.name.clone(), move |ch, qu| {
                ch.basic_publish(
                    "",
                    &qu[..],
                    true,
                    false,
                    amqp::protocol::basic::BasicProperties {
                        content_type: Some(content_type.clone()),
                        _type: Some(type_.clone()),
                        priority: Some(priority),
                        timestamp: Some(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()),
                        message_id: Some(id.clone()),
                        ..Default::default()
                    },
                    payload.to_vec(),
                )?;
                Ok(())
            });
        }
        Err(BAD_PROVIDER.into())
    }
}

pub trait Consumer: Sync + Send {
    fn consume(
        &self,
        ctx: &Context,
        id: &String,
        content_type: &String,
        priority: u8,
        payload: &[u8],
    ) -> Result<()>;
}
