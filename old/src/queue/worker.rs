use std::sync::Arc;

use super::super::{
    context::Context, plugins::nut::consumers::{SendMail, SEND_MAIL},
};
use super::consumer::Consumer;

pub fn new(ctx: &Arc<Context>) -> Consumer {
    let mut it = Consumer::new();
    it.register(s!(SEND_MAIL), Box::new(SendMail::new(Arc::clone(ctx))));
    it
}
