use frank_jwt::Algorithm;

use super::{cache, env::Config, errors::Result, jwt::Jwt, orm, queue, security::Encryptor};

pub struct Context {
    pub db: orm::Pool,
    pub cache: Box<cache::Cache>,
    pub queue: Box<queue::Queue>,
    pub encryptor: Encryptor,
    pub jwt: Jwt,
    pub config: Config,
}

impl Context {
    pub fn new(cfg: &Config) -> Result<Self> {
        Ok(Self {
            #[cfg(feature = "postgresql")]
            db: cfg.database.postgresql.open()?,
            #[cfg(feature = "mysql")]
            db: cfg.database.mysql.open()?,
            cache: cfg.cache.open()?,
            queue: cfg.queue.open()?,
            encryptor: Encryptor::new(cfg.secret_key()?.as_slice())?,
            jwt: Jwt::new(cfg.secret_key.clone(), Algorithm::HS512),
            config: cfg.clone(),
        })
    }
}
