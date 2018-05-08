use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use amqp::{self, protocol, Basic, Channel, Session, Table};
use log;
use serde::ser::Serialize;
use serde_json;
use uuid::Uuid;

use super::env;
use super::result::{Error, Result};

pub fn put<T: Serialize>(
    qu: &Queue,
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

pub trait Consumer: Send + Sync {
    fn run(&self, id: &String, content_type: &String, payload: &[u8]) -> Result<()>;
}

//-----------------------------------------------------------------------------

#[derive(Clone)]
pub struct Worker {
    consumers: Arc<HashMap<String, Box<Consumer>>>,
}

impl Worker {
    pub fn new(consumers: HashMap<String, Box<Consumer>>) -> Self {
        Self {
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
            Some(consumer) => consumer.run(id, content_type, payload),
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
        channel: &mut Channel,
        deliver: protocol::basic::Deliver,
        headers: protocol::basic::BasicProperties,
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

pub enum Queue {
    RabbitMQ((String, env::RabbitMQ)),
}

impl Queue {
    fn rabbitmq<F>(name: &String, cfg: &env::RabbitMQ, f: F) -> Result<()>
    where
        F: Fn(&mut Channel) -> Result<()>,
    {
        let mut ss = Session::new(cfg.options())?;
        let mut ch = ss.open_channel(1)?;
        ch.queue_declare(
            &name[..],
            false, // passive,
            true,  // durable
            false, // exclusive
            false, // auto_delete
            false, // nowait
            Table::new(),
        )?;

        f(&mut ch)?;

        ch.close(200, "Bye")?;
        ss.close(200, "Good Bye");

        Ok(())
    }
}

impl Queue {
    pub fn publish(
        &self,
        _type: &String,
        content_type: &String,
        priority: u8,
        payload: &[u8],
    ) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        match self {
            Queue::RabbitMQ((name, cfg)) => {
                log::info!("push task into queue {}@{}", id, name);
                Queue::rabbitmq(name, cfg, |ch| {
                    ch.basic_publish(
                        "",
                        name,
                        true,
                        false,
                        protocol::basic::BasicProperties {
                            content_type: Some(content_type.to_string()),
                            _type: Some(_type.to_string()),
                            priority: Some(priority),
                            timestamp: Some(
                                SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
                            ),
                            message_id: Some(id.clone()),
                            ..Default::default()
                        },
                        payload.to_vec(),
                    )?;
                    return Ok(());
                })
            }
        }
    }

    // fn consume(&self, name: String, worker: Worker) -> Result<()> {
    //     self.open(|ch| {
    //         let it = ch.basic_consume(
    //             worker.clone(),
    //             self.queue.clone(),
    //             name.clone(), // consumer_tag
    //             false,        // no_local
    //             false,        // no_ack
    //             false,        // exclusive
    //             false,        // nowait
    //             Table::new(),
    //         )?;
    //         log::info!("Starting consumer {:?}", it);
    //         ch.start_consuming();
    //         Ok(())
    //     })
    // }
}
