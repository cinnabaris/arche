use std::time::{SystemTime, UNIX_EPOCH};

use amqp::{self, protocol, Basic, Channel, Session, Table};
use log;
use rocket::config::Environment;
use serde::ser::Serialize;
use serde_json;
use uuid::Uuid;

use super::nut::workers::{SendEmail, SEND_EMAIL};
use super::result::{Error, Result};
use super::{orm, security};

pub type Queue = RabbitMQ;

pub fn new(url: String, queue: String) -> Queue {
    return RabbitMQ {
        url: url,
        queue: queue,
    };
}

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

#[derive(Clone)]
pub struct Consumer {
    name: String,
    env: Environment,
    db: orm::Pool,
    encryptor: security::Encryptor,
}

impl Consumer {
    fn run(
        &self,
        _type: &String,
        _id: &String,
        _content_type: &String,
        payload: &[u8],
    ) -> Result<()> {
        match &_type[..] {
            SEND_EMAIL => self.send_email(payload, self.env.is_prod()),
            _ => Err(Error::WithDescription(format!("bad task type {}", _type))),
        }
    }
}

impl amqp::Consumer for Consumer {
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
                    match self.run(&_type, &id, &content_type, body.as_slice()) {
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

pub trait Provider: Send + Sync {
    fn publish(
        &self,
        _type: &String,
        content_type: &String,
        priority: u8,
        payload: &[u8],
    ) -> Result<()>;
    fn consume(&self, name: &String, consumer: Consumer) -> Result<()>;
}

pub struct RabbitMQ {
    url: String,
    queue: String,
}

impl RabbitMQ {
    fn open<F>(&self, f: F) -> Result<()>
    where
        F: Fn(&mut Channel) -> Result<()>,
    {
        let mut ss = try!(Session::open_url(&self.url));
        let mut ch = try!(ss.open_channel(1));
        try!(ch.queue_declare(
            &self.queue[..],
            false, // passive,
            true,  // durable
            false, // exclusive
            false, // auto_delete
            false, // nowait
            Table::new(),
        ));

        try!(f(&mut ch));

        try!(ch.close(200, "Bye"));
        ss.close(200, "Good Bye");

        Ok(())
    }
}

impl Provider for RabbitMQ {
    fn consume(&self, name: &String, consumer: Consumer) -> Result<()> {
        self.open(|ch| {
            let consumer = consumer.clone();
            let it = ch.basic_consume(
                consumer,
                self.queue.clone(),
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

    fn publish(
        &self,
        _type: &String,
        content_type: &String,
        priority: u8,
        payload: &[u8],
    ) -> Result<()> {
        return self.open(|ch| {
            let id = Uuid::new_v4().to_string();
            log::info!("push task into queue {}@{}", id, self.queue);
            try!(ch.basic_publish(
                "",
                &self.queue[..],
                true,
                false,
                protocol::basic::BasicProperties {
                    content_type: Some(content_type.to_string()),
                    _type: Some(_type.to_string()),
                    priority: Some(priority),
                    timestamp: Some(try!(SystemTime::now().duration_since(UNIX_EPOCH)).as_secs()),
                    message_id: Some(id),
                    ..Default::default()
                },
                payload.to_vec(),
            ));
            return Ok(());
        });
    }
}
