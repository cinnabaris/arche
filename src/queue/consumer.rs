use std::sync::Arc;

use super::super::{context::Context, errors::Result};

pub struct Consumer {
    _ctx: Arc<Context>,
}

impl Consumer {
    pub fn new(ctx: Arc<Context>) -> Self {
        Self { _ctx: ctx }
    }

    pub fn consume(
        &self,
        id: &String,
        type_: &String,
        _content_type: &String,
        _priority: u8,
        _payload: &[u8],
    ) -> Result<()> {
        info!("receive message {}@{}", id, type_);
        match type_ {
            // TODO
            t => Err(format!("can't find consumer for {}", t).into()),
        }
    }
}
