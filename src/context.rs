use frank_jwt::Algorithm;

use super::{
    cache::Cache, env::Config, errors::Result, jwt::Jwt, orm, queue::Producer, storage::Storage,
    utils::Encryptor,
};

pub struct Context {
    pub db: orm::Pool,
    pub cache: Cache,
    pub queue: Producer,
    pub storage: Storage,
    pub encryptor: Encryptor,
    pub config: Config,
    pub jwt: Jwt,
}

impl Context {
    pub fn new(cfg: &Config) -> Result<Self> {
        Ok(Self {
            db: cfg.database.open()?,
            cache: cfg.cache.open()?,
            queue: Producer::new(cfg.queue.clone()),
            encryptor: Encryptor::new(cfg.secret_key()?.as_slice())?,
            storage: Storage::new(cfg.storage.clone()),
            jwt: Jwt::new(cfg.secret_key.clone(), Algorithm::HS512),
            config: cfg.clone(),
        })
    }
}
