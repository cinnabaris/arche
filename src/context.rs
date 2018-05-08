use frank_jwt::Algorithm;

use super::cache::Cache;
use super::env;
use super::jwt::Jwt;
use super::queue::Queue;
use super::repositories::Pool as Repository;
use super::result::{Error, Result};
use super::security::{Security, Sodium};

pub struct Context {
    pub repository: Repository,
    pub cache: Cache,
    pub queue: Queue,
    pub security: Box<Security>,
    pub jwt: Jwt,
}

impl Context {
    pub fn new(cfg: &env::Config) -> Result<Self> {
        Ok(Self {
            repository: Self::open_database(&cfg.database)?,
            cache: Self::open_cache(&cfg.cache)?,
            queue: Self::open_queue(&cfg.queue)?,
            security: Box::new(Sodium::new(cfg.secret_key()?.as_slice())?),
            jwt: Jwt::new(cfg.secret_key()?.as_slice(), Algorithm::HS512),
        })
    }

    fn open_database(cfg: &env::Database) -> Result<Repository> {
        if let Some(ref c) = cfg.postgresql {
            return Ok(Repository::PostgreSql(c.pool()?));
        }
        Err(Error::WithDescription(s!(
            "unsupport messaging database provider"
        )))
    }

    fn open_cache(cfg: &env::Cache) -> Result<Cache> {
        if let Some(ref c) = cfg.redis {
            return Ok(Cache::Redis((cfg.namespace.clone(), c.pool()?)));
        }
        Err(Error::WithDescription(s!(
            "unsupport messaging cache provider"
        )))
    }

    fn open_queue(cfg: &env::Queue) -> Result<Queue> {
        if let Some(ref c) = cfg.rabbitmq {
            return Ok(Queue::RabbitMQ((cfg.name.clone(), c.clone())));
        }
        Err(Error::WithDescription(s!(
            "unsupport messaging queue provider"
        )))
    }
}
