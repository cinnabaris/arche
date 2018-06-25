use std::sync::Arc;

use super::super::{
    context::Context,
    plugins::nut::consumers::{SendMail, SEND_MAIL},
};
use super::consumer::Consumer;

pub fn new(ctx: &Arc<Context>) -> Consumer {
    let mut it = Consumer::new();
    it.register(s!(SEND_MAIL), Box::new(SendMail::new(Arc::clone(ctx))));
    it
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
        Err(Error::WithDescription(format!(
            "can't find consumer for {}@{}",
            id, type_
        )))
    }
}

use std::collections::HashMap;

// fn consume(&self, name: String, worker: &Arc<Context>) -> Result<()>;
// fn consume(&self, name: String, ctx: &Arc<Context>) -> Result<()> {
//     self.open(|ch| {
//         let worker = super::worker::new(&Arc::clone(ctx));
//
//         let it = ch.basic_consume(
//             worker,
//             self.queue.clone(),
//             name.clone(), // consumer_tag
//             false,        // no_local
//             false,        // no_ack
//             false,        // exclusive
//             false,        // nowait
//             amqp::Table::new(),
//         )?;
//         log::info!("Starting consumer {:?}", it);
//         ch.start_consuming();
//         Ok(())
//     })
// }

// impl amqp::Consumer for Consumer {
//     fn handle_delivery(
//         &mut self,
//         channel: &mut amqp::Channel,
//         deliver: amqp::protocol::basic::Deliver,
//         headers: amqp::protocol::basic::BasicProperties,
//         body: Vec<u8>,
//     ) {
//         if let Some(ref _type) = headers._type {
//             if let Some(ref content_type) = headers.content_type {
//                 if let Some(ref id) = headers.message_id {
//                     if let Some(priority) = headers.priority {
//                         log::info!("receive message: {}@{}", id, _type);
//                         match self.consume(&id, &_type, &content_type, priority, body.as_slice()) {
//                             Ok(_) => match channel.basic_ack(deliver.delivery_tag, true) {
//                                 Ok(_) => {}
//                                 Err(e) => log::error!("{:?}", e),
//                             },
//                             Err(e) => log::error!("{:?}", e),
//                         };
//                         return;
//                     }
//                 }
//             }
//         }
//         log::error!("bad task message header: {:?}", headers);
//     }
// }
