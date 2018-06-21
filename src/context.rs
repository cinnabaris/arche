use frank_jwt::Algorithm;

use super::{env, jwt::Jwt, result::Result, security::Encryptor};

#[cfg(feature = "ch-redis")]
use super::cache::redis::Cache;
#[cfg(feature = "mysql")]
use super::dao::mysql::{Config, Dao};
#[cfg(feature = "postgresql")]
use super::dao::postgresql::Dao;
#[cfg(feature = "mq-rabbit")]
use super::queue::rabbitmq::Queue;

pub struct Context {
    pub dao: Dao,
    pub cache: Cache,
    pub queue: Queue,
    pub encryptor: Encryptor,
    pub jwt: Jwt,
    pub config: env::Config,
}

impl Context {
    pub fn new(cfg: &env::Config) -> Result<Self> {
        Ok(Self {
            dao: Self::open_dao(&cfg.database)?,
            cache: Self::open_cache(&cfg.cache)?,
            queue: Self::open_queue(&cfg.queue)?,
            encryptor: Encryptor::new(cfg.secret_key()?.as_slice())?,
            jwt: Jwt::new(cfg.secret_key.clone(), Algorithm::HS512),
            config: cfg.clone(),
        })
    }

    #[cfg(feature = "postgresql")]
    fn open_dao(cfg: &env::Database) -> Result<Dao> {
        Dao::new(&cfg.postgresql.url())
    }

    #[cfg(feature = "mysql")]
    fn open_dao(cfg: &env::Database) -> Result<Dao> {
        Dao::new(&cfg.mysql.url())
    }

    #[cfg(feature = "ch-redis")]
    fn open_cache(cfg: &env::Cache) -> Result<Cache> {
        Ok(Cache::new(cfg.namespace.clone(), cfg.redis.pool()?))
    }
    #[cfg(feature = "mq-rabbit")]
    fn open_queue(cfg: &env::Queue) -> Result<Queue> {
        Ok(Queue::new(cfg.name.clone(), cfg.rabbitmq.options()))
    }
}
