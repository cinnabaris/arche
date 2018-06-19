use frank_jwt::Algorithm;

use super::{
    dao, env, jwt::Jwt, result::{Error, Result}, security::Encryptor,
};

#[cfg(feature = "ch-redis")]
use super::cache::redis::Cache;
#[cfg(feature = "mq-rabbit")]
use super::queue::rabbitmq::Queue;

pub struct Context {
    db: DbPool,
    pub cache: Cache,
    pub queue: Queue,
    pub encryptor: Encryptor,
    pub jwt: Jwt,
    pub config: env::Config,
}

impl Context {
    pub fn new(cfg: &env::Config) -> Result<Self> {
        Ok(Self {
            db: Self::open_database(&cfg.database)?,
            cache: Self::open_cache(&cfg.cache)?,
            queue: Self::open_queue(&cfg.queue)?,
            encryptor: Encryptor::new(cfg.secret_key()?.as_slice())?,
            jwt: Jwt::new(cfg.secret_key.clone(), Algorithm::HS512),
            config: cfg.clone(),
        })
    }

    pub fn db(&self) -> Result<Db> {
        self.db.get()
    }

    fn open_database(cfg: &env::Database) -> Result<DbPool> {
        if let Some(ref pg) = cfg.postgresql {
            return Ok(DbPool::new(&pg.url())?);
        }
        Err(Error::WithDescription(s!("can't open database")))
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
