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

fn consume(
    ctx: Arc<Context>,
    id: &String,
    type_: &String,
    content_type: &String,
    priority: u8,
    payload: &[u8],
) -> Result<()> {
    log::info!("receive message {}@{}", id, type_);
    match CONSUMERS.get(&type_[..]) {
        Some(c) => c.consume(&ctx, id, content_type, priority, payload),
        None => Err(format!("can't find consumer for {}", type_).into()),
    }
}

pub fn start(cfg: &Config, ctx: Arc<Context>) -> Result<()> {
    let name = cfg.name.clone();
    if let Some(ref cfg) = cfg.rabbitmq {
        return cfg.open(name, move |ch, qu| -> Result<()> {
            log::info!("starting worker thread");
            for rst in ch.basic_get(qu, false) {
                if let Some(ref type_) = rst.headers._type {
                    if let Some(ref content_type) = rst.headers.content_type {
                        if let Some(ref id) = rst.headers.message_id {
                            if let Some(priority) = rst.headers.priority {
                                consume(
                                    Arc::clone(&ctx),
                                    &id,
                                    &type_,
                                    &content_type,
                                    priority,
                                    rst.body.as_slice(),
                                )?;
                                rst.ack();
                                continue;
                            }
                        }
                    }
                }
                return Err(format!("bad task message header: {:?}", rst.headers).into());
            }
            Ok(())
        });
    }
    Err(BAD_PROVIDER.into())
}
