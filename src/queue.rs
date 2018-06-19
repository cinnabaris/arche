use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use amqp::{self, Basic as AmqpBasic};
use log;
use serde::ser::Serialize;
use serde_json;
use uuid::Uuid;

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

impl amqp::Consumer for Worker {
    fn handle_delivery(
        &mut self,
        channel: &mut amqp::Channel,
        deliver: amqp::protocol::basic::Deliver,
        headers: amqp::protocol::basic::BasicProperties,
        body: Vec<u8>,
    ) {
        if let Some(ref _type) = headers._type {
            if let Some(ref content_type) = headers.content_type {
                if let Some(ref id) = headers.message_id {
                    log::info!("receive message: {}@{}", id, _type);
                    match self.handle(&_type, &id, &content_type, body.as_slice()) {
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
        log::error!("bad task message header: {:?}", headers);
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

pub struct RabbitMQ {
    name: String,
    cfg: Arc<amqp::Options>,
}

impl RabbitMQ {
    pub fn new(name: String, cfg: amqp::Options) -> Self {
        Self {
            name: name,
            cfg: Arc::new(cfg),
        }
    }
    fn open<F>(&self, f: F) -> Result<()>
    where
        F: Fn(&mut amqp::Channel) -> Result<()>,
    {
        let cfg = Arc::clone(&self.cfg);

        match Arc::try_unwrap(cfg) {
            Ok(cfg) => {
                let mut ss = amqp::Session::new(cfg)?;
                let mut ch = ss.open_channel(1)?;
                ch.queue_declare(
                    &self.name[..],
                    false, // passive,
                    true,  // durable
                    false, // exclusive
                    false, // auto_delete
                    false, // nowait
                    amqp::Table::new(),
                )?;

                f(&mut ch)?;

                ch.close(200, "Bye")?;
                ss.close(200, "Good Bye");

                Ok(())
            }
            Err(_) => Err(Error::WithDescription(s!("can't get rabbitmq options"))),
        }
    }
}

impl Queue for RabbitMQ {
    fn publish(
        &self,
        _type: &String,
        content_type: &String,
        priority: u8,
        payload: &[u8],
    ) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        log::info!("push task into queue {}@{}", id, self.name);
        self.open(|ch| {
            ch.basic_publish(
                "",
                &self.name[..],
                true,
                false,
                amqp::protocol::basic::BasicProperties {
                    content_type: Some(content_type.to_string()),
                    _type: Some(_type.to_string()),
                    priority: Some(priority),
                    timestamp: Some(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()),
                    message_id: Some(id.clone()),
                    ..Default::default()
                },
                payload.to_vec(),
            )?;
            return Ok(());
        })
    }

    fn consume(&self, name: String, worker: Arc<Worker>) -> Result<()> {
        self.open(|ch| {
            let worker = Arc::clone(&worker);
            match Arc::try_unwrap(worker) {
                Ok(worker) => {
                    let it = ch.basic_consume(
                        worker,
                        self.name.clone(),
                        name.clone(), // consumer_tag
                        false,        // no_local
                        false,        // no_ack
                        false,        // exclusive
                        false,        // nowait
                        amqp::Table::new(),
                    )?;
                    log::info!("Starting consumer {:?}", it);
                    ch.start_consuming();
                    Ok(())
                }
                Err(_) => Err(Error::WithDescription(s!("can't get worker"))),
            }
        })
    }
}
