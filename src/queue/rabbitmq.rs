use std::time::{SystemTime, UNIX_EPOCH};

use amqp::{self, Basic, Options};
use serde::ser::Serialize;
use serde_json;
use uuid::Uuid;

use super::super::errors::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    #[serde(rename = "virtual")]
    pub virtual_: String,
    pub name: String,
}

impl Config {
    pub fn options(&self) -> Options {
        Options {
            host: self.host.clone(),
            port: self.port,
            login: self.user.clone(),
            password: self.password.clone(),
            vhost: self.virtual_.clone(),
            ..Default::default()
        }
    }
}

pub struct Producer {
    cfg: Config,
}

impl Producer {
    pub fn new(cfg: Config) -> Self {
        Self { cfg: cfg }
    }
}

impl super::Provider for Producer {
    fn push<T: Serialize>(
        &self,
        type_: &String,
        content_type: &String,
        priority: u8,
        payload: &T,
    ) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        info!("push task into queue {}@{}", id, self.cfg.name);
        open(self.cfg.name.clone(), self.cfg.options(), |ch, qu| {
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
                serde_json::to_vec(payload)?,
            )?;
            Ok(())
        })
    }
}

impl amqp::Consumer for super::consumer::Consumer {
    fn handle_delivery(
        &mut self,
        channel: &mut amqp::Channel,
        deliver: amqp::protocol::basic::Deliver,
        headers: amqp::protocol::basic::BasicProperties,
        body: Vec<u8>,
    ) {
        if let Some(ref type_) = headers._type {
            if let Some(ref content_type) = headers.content_type {
                if let Some(ref id) = headers.message_id {
                    if let Some(priority) = headers.priority {
                        match self.consume(&id, &type_, &content_type, priority, body.as_slice()) {
                            Ok(_) => match channel.basic_ack(deliver.delivery_tag, true) {
                                Ok(_) => {}
                                Err(e) => error!("{:?}", e),
                            },
                            Err(e) => error!("{:?}", e),
                        };
                        return;
                    }
                }
            }
        }
        error!("bad task message header: {:?}", headers);
    }
}

pub fn open<F>(queue: String, opt: Options, f: F) -> Result<()>
where
    F: Fn(&mut amqp::Channel, &String) -> Result<()>,
{
    let mut ss = amqp::Session::new(opt)?;
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
