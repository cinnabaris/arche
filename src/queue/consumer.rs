use std::collections::HashMap;

use super::super::result::{Error, Result};

pub trait Handler: Send + Sync {
    fn handle(
        &self,
        id: &String,
        type_: &String,
        content_type: &String,
        priority: u8,
        payload: &[u8],
    ) -> Result<()>;
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
