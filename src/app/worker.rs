use std::sync::Arc;

use amqp::{self, Basic};
use log;

use super::super::{
    context::Context,
    errors::Result,
    queue::{rabbitmq, Config, Consumer, BAD_PROVIDER},
};

pub fn start(cfg: &Config, ctx: Arc<Context>) -> Result<()> {
    let name = cfg.name.clone();
    if let Some(ref cfg) = cfg.rabbitmq {
        return rabbitmq(name, cfg.options(), |ch, qu| -> Result<()> {
            let it = Consumer::new(ctx.clone());
            log::info!("starting worker thread");
            let name = ch.basic_consume(
                it,
                &qu[..],
                "",
                false,
                true,
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
