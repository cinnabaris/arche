use std::collections::HashMap;
use std::sync::Arc;

use amqp::{Basic, Table};
use nut::{consumers::send_mail, context::Context, errors::Result, queue::consumer::Handler};

pub fn consume(ctx: &Arc<Context>) -> Result<()> {
    if let Some(ref cfg) = ctx.config.queue.rabbitmq {
        return cfg.open(&ctx.config.queue.name, |ch| -> Result<()> {
            let mut it = Consumer::new();
            it.register(
                String::from(send_mail::NAME),
                Box::new(send_mail::Consumer::new(Arc::clone(ctx))),
            );
            info!("starting worker thread");
            let name = ch.basic_consume(
                it,
                &ctx.config.queue.name[..],
                "",
                false,
                true,
                false,
                false,
                Table::new(),
            )?;
            info!("starting consumer {}", name);
            ch.start_consuming();
            Ok(())
        });
    }
    Err("bad messaging queue provider".into())
}

pub struct Consumer {
    handlers: HashMap<String, Box<Handler>>,
}

impl Consumer {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }
    pub fn register(&mut self, type_: String, handler: Box<Handler>) {
        self.handlers.insert(type_, handler);
    }
    pub fn consume(
        &self,
        id: &String,
        type_: &String,
        content_type: &String,
        priority: u8,
        payload: &[u8],
    ) -> Result<()> {
        for (t, h) in &self.handlers {
            if t == type_ {
                return h.handle(id, type_, content_type, priority, payload);
            }
        }
        Err("can't find consumer".into())
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
        if let Some(ref _type) = headers._type {
            if let Some(ref content_type) = headers.content_type {
                if let Some(ref id) = headers.message_id {
                    if let Some(priority) = headers.priority {
                        info!("receive message: {}@{}", id, _type);
                        match self.consume(&id, &_type, &content_type, priority, body.as_slice()) {
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
