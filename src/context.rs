use super::{cache, orm, queue, storage, utils};

pub struct Context {
    pub db: orm::Pool,
    pub cache: cache::Cache,
    pub queue: queue::Producer,
    pub storage: storage::Storage,
    pub encryptor: utils::Encryptor,
}
