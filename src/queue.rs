use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use amqp::{self, Basic};
use log;
use mime;
use serde_json;
use uuid::Uuid;

use super::{context::Context, errors::Result, plugins::nut::consumers::send_mail};

pub const BAD_PROVIDER: &'static str = "bad messing queue provider";

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
    pub fn options(&self) -> amqp::Options {
        amqp::Options {
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

    pub fn push(&self, type_: &String, priority: u8, payload: &serde_json::Value) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        log::info!("push task into queue {}@{}", id, self.cfg.name);
        if let Some(ref cfg) = self.cfg.rabbitmq {
            return rabbitmq(self.cfg.name.clone(), cfg.options(), move |ch, qu| {
                ch.basic_publish(
                    "",
                    &qu[..],
                    true,
                    false,
                    amqp::protocol::basic::BasicProperties {
                        content_type: Some(format!("{}", mime::APPLICATION_JSON)),
                        _type: Some(type_.clone()),
                        priority: Some(priority),
                        timestamp: Some(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()),
                        message_id: Some(id.clone()),
                        ..Default::default()
                    },
                    serde_json::to_vec(payload)?,
                )?;
                Ok(())
            });
        }
        Err(BAD_PROVIDER.into())
    }
}

pub struct Consumer {
    ctx: Arc<Context>,
}

impl Consumer {
    pub fn new(ctx: Arc<Context>) -> Self {
        Self { ctx: ctx }
    }

    fn consume(
        &self,
        id: &String,
        type_: &String,
        _content_type: &String,
        _priority: u8,
        payload: &[u8],
    ) -> Result<()> {
        log::info!("receive message {}@{}", id, type_);
        match &type_[..] {
            send_mail::NAME => send_mail::handle(&self.ctx, payload),
            t => Err(format!("can't find consumer for {}", t).into()),
        }
    }
}

impl amqp::Consumer for Consumer {
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
                                Err(e) => log::error!("{:?}", e),
                            },
                            Err(e) => log::error!("{:?}", e),
                        };
                        return;
                    }
                }
            }
        }
        log::error!("bad task message header: {:?}", headers);
    }
}

pub fn rabbitmq<F>(queue: String, opt: amqp::Options, f: F) -> Result<()>
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
