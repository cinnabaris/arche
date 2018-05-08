use frank_jwt::Algorithm;

use super::cache::Cache;
use super::env;
use super::jwt::Jwt;
use super::queue::Queue;
use super::repositories::Pool as Repository;
use super::result::Result;
use super::security::Encryptor;

pub struct Context {
    pub repository: Repository,
    pub cache: Cache,
    pub queue: Queue,
    pub encryptor: Encryptor,
    pub jwt: Jwt,
}

impl Context {
    pub fn new(cfg: &env::Config) -> Result<Self> {
        Ok(Self {
            repository: Self::open_database(&cfg.database)?,
            cache: Self::open_cache(&cfg.cache)?,
            queue: Self::open_queue(&cfg.queue)?,
            encryptor: Encryptor::new(cfg.secret_key()?.as_slice())?,
            jwt: Jwt::new(cfg.secret_key()?.as_slice(), Algorithm::HS512),
        })
    }

    #[cfg(feature = "postgresql")]
    fn open_database(cfg: &env::Database) -> Result<Repository> {
        Ok(Repository::PostgreSql(cfg.postgresql.pool()?))
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
