use super::super::{
    context::Context, plugins::nut::consumers::{SendEmail, SEND_EMAIL}, result::{Error, Result},
};

pub trait Worker {
    fn handle(
        &self,
        type_: &String,
        id: &String,
        content_type: &String,
        payload: &[u8],
    ) -> Result<()>;
}

impl Worker for Context {
    fn handle(
        &self,
        type_: &String,
        _id: &String,
        _content_type: &String,
        payload: &[u8],
    ) -> Result<()> {
        match &type_[..] {
            SEND_EMAIL => self.send_email(payload),
            _ => Err(Error::WithDescription(format!(
                "can't find consumer for {:?}",
                type_
            ))),
        }
    }
}
