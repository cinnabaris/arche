mod mysql;
mod postgresql;

pub trait Repository: Send + Sync {}
