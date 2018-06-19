use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use amqp::{self, Basic, Options};
use log;
use uuid::Uuid;

use super::super::result::{Error, Result};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    #[serde(rename = "virtual")]
    pub _virtual: String,
}

impl Config {
    pub fn options(&self) -> Options {
        Options {
            host: self.host.clone(),
            port: self.port,
            login: self.user.clone(),
            password: self.password.clone(),
            vhost: self._virtual.clone(),
            ..Default::default()
        }
    }
}

impl amqp::Consumer for super::Worker {
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

pub struct Queue {
    name: String,
    cfg: Arc<amqp::Options>,
}

impl Queue {
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

impl super::Queue for Queue {
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

    fn consume(&self, name: String, worker: Arc<super::Worker>) -> Result<()> {
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
