use std::sync::Arc;

#[cfg(feature = "mq-rabbit")]
use amqp::{Basic, Table};

use super::super::{
    context::Context,
    errors::Result,
    queue::{self, consumer::Consumer, Config},
};

#[cfg(feature = "mq-rabbit")]
pub fn start(cfg: &Config, ctx: Arc<Context>) -> Result<()> {
    queue::rabbitmq::open(cfg.name.clone(), cfg.options(), |ch, qu| -> Result<()> {
        let it = Consumer::new(ctx.clone());
        info!("starting worker thread");
        let name = ch.basic_consume(it, &qu[..], "", false, true, false, false, Table::new())?;
        info!("starting consumer {}", name);
        ch.start_consuming();
        Ok(())
    })
}
