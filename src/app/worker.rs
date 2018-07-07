use std::collections::HashMap;
use std::sync::Arc;

use amqp::Basic;
use log;

use super::super::{
    context::Context,
    errors::Result,
    plugins::nut::consumers::send_mail,
    queue::{Config, Consumer, BAD_PROVIDER},
};

lazy_static! {
    static ref CONSUMERS: HashMap<&'static str, Box<Consumer>> = {
        let mut m: HashMap<&'static str, Box<Consumer>> = HashMap::new();
        m.insert(send_mail::NAME, Box::new(send_mail::Consumer {}));
        m
    };
}

pub struct Worker {
    ctx: Arc<Context>,
}

impl Worker {
    pub fn new(ctx: Arc<Context>) -> Self {
        Self { ctx: ctx }
    }
    fn consume(
        &self,
        id: &String,
        type_: &String,
        content_type: &String,
        priority: u8,
        payload: &[u8],
    ) -> Result<()> {
        log::info!("receive message {}@{}", id, type_);
        match CONSUMERS.get(&type_[..]) {
            Some(c) => c.consume(&self.ctx, id, content_type, priority, payload),
            None => Err(format!("can't find consumer for {}", type_).into()),
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
        if let Some(ref type_) = headers._type {
            if let Some(ref content_type) = headers.content_type {
                if let Some(ref id) = headers.message_id {
                    if let Some(priority) = headers.priority {
                        match self.consume(&id, &type_, &content_type, priority, body.as_slice()) {
                            Ok(_) => if let Err(e) = channel.basic_ack(deliver.delivery_tag, false)
                            {
                                log::error!("ack {:?}", e);
                            },
                            Err(e) => log::error!("consume {:?}", e),
                        };
                        return;
                    }
                }
            }
        }
        log::error!("bad task message header: {:?}", headers);
    }
}

pub fn start(cfg: &Config, ctx: Arc<Context>) -> Result<()> {
    let name = cfg.name.clone();
    if let Some(ref cfg) = cfg.rabbitmq {
        return cfg.open(name, move |ch, qu| -> Result<()> {
            let worker = Worker::new(Arc::clone(&ctx));
            let name = ch.basic_consume(
                worker,
                &qu[..],
                "",
                false,
                false,
                false,
                false,
                amqp::Table::new(),
            )?;
            log::info!("starting consumer {}", name);
            ch.start_consuming();
            Ok(())
        });
    }
    Err(BAD_PROVIDER.into())
}
