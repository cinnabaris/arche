use frank_jwt::Algorithm;

use super::cache::Cache;
use super::env;
use super::jwt::Jwt;
use super::orm::{Pool as DbPool, PooledConnection as Db};
use super::queue::Queue;
use super::result::Result;
use super::security::Encryptor;

pub struct Context {
    dbp: DbPool,
    pub cache: Cache,
    pub queue: Queue,
    pub encryptor: Encryptor,
    pub jwt: Jwt,
}

impl Context {
    pub fn new(cfg: &env::Config) -> Result<Self> {
        Ok(Self {
            dbp: Self::open_database(&cfg.database)?,
            cache: Self::open_cache(&cfg.cache)?,
            queue: Self::open_queue(&cfg.queue)?,
            encryptor: Encryptor::new(cfg.secret_key()?.as_slice())?,
            jwt: Jwt::new(cfg.secret_key()?.as_slice(), Algorithm::HS512),
        })
    }
    pub fn db(&self) -> Result<Db> {
        self.dbp.get()
    }

    #[cfg(feature = "postgresql")]
    fn open_database(cfg: &env::Database) -> Result<DbPool> {
        Ok(DbPool::new(&cfg.postgresql.url())?)
    }

    #[cfg(feature = "cache-redis")]
    fn open_cache(cfg: &env::Cache) -> Result<Cache> {
        return Ok(Cache::new(cfg.namespace.clone(), cfg.redis.pool()?));
    }

    #[cfg(feature = "rabbitmq")]
    fn open_queue(cfg: &env::Queue) -> Result<Queue> {
        return Ok(Queue::RabbitMQ((cfg.name.clone(), cfg.rabbitmq.clone())));
    }
}
