use std::sync::Arc;

use super::super::{context::Context, errors::Result, plugins::nut::consumers::send_mail};

pub struct Consumer {
    ctx: Arc<Context>,
}

impl Consumer {
    pub fn new(ctx: Arc<Context>) -> Self {
        Self { ctx: ctx }
    }

    pub fn consume(
        &self,
        id: &String,
        type_: &String,
        _content_type: &String,
        _priority: u8,
        payload: &[u8],
    ) -> Result<()> {
        info!("receive message {}@{}", id, type_);
        match &type_[..] {
            send_mail::NAME => send_mail::handle(&self.ctx, payload),
            t => Err(format!("can't find consumer for {}", t).into()),
        }
    }
}
