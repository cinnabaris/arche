use std::time::{SystemTime, UNIX_EPOCH};

use amqp::{self, protocol, Basic, Channel, Session, Table};
use log;
use uuid::Uuid;

use super::super::env;
use super::super::result::Result;
use super::{Queue, Worker};

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

#[derive(Clone)]
pub struct RabbitMQ {
    name: String,
    cfg: env::RabbitMQ,
}

impl RabbitMQ {
    pub fn new(name: String, cfg: env::RabbitMQ) -> Self {
        Self {
            name: name,
            cfg: cfg,
        }
    }
    fn open<F>(&self, f: F) -> Result<()>
    where
        F: Fn(&mut Channel) -> Result<()>,
    {
        let mut ss = Session::new(self.cfg.options())?;
        let mut ch = ss.open_channel(1)?;
        ch.queue_declare(
            &self.name[..],
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
                protocol::basic::BasicProperties {
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

    fn consume(&self, name: String, worker: Worker) -> Result<()> {
        self.open(|ch| {
            let it = ch.basic_consume(
                worker.clone(),
                self.name.clone(),
                name.clone(), // consumer_tag
                false,        // no_local
                false,        // no_ack
                false,        // exclusive
                false,        // nowait
                Table::new(),
            )?;
            log::info!("Starting consumer {:?}", it);
            ch.start_consuming();
            Ok(())
        })
    }
}
