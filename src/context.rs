use frank_jwt::Algorithm;

use super::{
    cache::{self, Cache}, env, jwt::Jwt, orm::{Pool as DbPool, PooledConnection as Db},
    queue::{self, Queue}, result::{Error, Result}, security::Encryptor,
};

pub struct Context {
    db: DbPool,
    pub cache: Box<Cache>,
    pub queue: Box<Queue>,
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

    fn open_cache(cfg: &env::Cache) -> Result<Box<Cache>> {
        if let Some(ref re) = cfg.redis {
            return Ok(Box::new(cache::Redis::new(
                cfg.namespace.clone(),
                re.pool()?,
            )));
        }
        Err(Error::WithDescription(s!("can't open cache")))
    }

    fn open_queue(cfg: &env::Queue) -> Result<Box<Queue>> {
        if let Some(ref mq) = cfg.rabbitmq {
            return Ok(Box::new(queue::RabbitMQ::new(
                cfg.name.clone(),
                mq.options(),
            )));
        }
        Err(Error::WithDescription(s!("can't open messing queue")))
    }
}
