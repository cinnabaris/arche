use super::super::{context::Context, errors::Result};

pub struct Consumer {
    ctx: Context,
}

impl Consumer {
    pub fn new(ctx: Context) -> Self {
        Self { ctx: ctx }
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
