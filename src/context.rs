use frank_jwt::Algorithm;

#[cfg(feature = "ch-redis")]
use super::cache::redis::Cache;
#[cfg(feature = "mq-rabbit")]
use super::queue::rabbitmq::Queue;
#[cfg(feature = "sodium")]
use super::security::sodium::SecretBox;
use super::{dao::Pool as Database, env, jwt::Jwt, result::Result};

#[derive(Clone)]
pub struct Context {
    pub db: Database,
    pub cache: Cache,
    pub queue: Queue,
    pub secret_box: SecretBox,
    pub jwt: Jwt,
    pub config: env::Config,
}

impl Context {
    pub fn new(cfg: &env::Config) -> Result<Self> {
        Ok(Self {
            db: Self::open_dao(&cfg.database)?,
            cache: Self::open_cache(&cfg.cache)?,
            queue: Self::open_queue(&cfg.queue)?,
            secret_box: SecretBox::new(cfg.secret_key()?.as_slice())?,
            jwt: Jwt::new(cfg.secret_key.clone(), Algorithm::HS512),
            config: cfg.clone(),
        })
    }

    #[cfg(feature = "postgresql")]
    fn open_dao(cfg: &env::Database) -> Result<Database> {
        cfg.postgresql.open()
    }

    #[cfg(feature = "mysql")]
    fn open_dao(cfg: &env::Database) -> Result<Database> {
        cfg.mysql.open()
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
