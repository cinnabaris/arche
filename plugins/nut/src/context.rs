use frank_jwt::Algorithm;

use super::{
    cache::Cache, env::Config, errors::Result, jwt::Jwt, orm::Pool as Database, queue::Queue,
    security::sodium::Encryptor,
};

pub struct Context {
    pub db: Database,
    pub cache: Cache,
    pub queue: Queue,
    pub encryptor: Encryptor,
    pub jwt: Jwt,
    pub config: Config,
}

impl Context {
    pub fn new(cfg: &Config) -> Result<Self> {
        Ok(Self {
            db: Database::new(&cfg.database)?,
            cache: Cache::new(&cfg.cache)?,
            queue: Queue::new(&cfg.queue)?,
            encryptor: Encryptor::new(cfg.secret_key()?.as_slice())?,
            jwt: Jwt::new(cfg.secret_key.clone(), Algorithm::HS512),
            config: cfg.clone(),
        })
    }
}
