#[cfg(feature = "mysql")]
use diesel::mysql::MysqlConnection;
#[cfg(feature = "postgresql")]
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use frank_jwt::Algorithm;
use r2d2::Pool;

use super::{env, jwt::Jwt, result::Result};

#[cfg(feature = "sodium")]
use super::security::sodium::{HashBox, SecretBox};

#[cfg(feature = "ch-redis")]
use super::cache::redis::Cache;
#[cfg(feature = "mq-rabbit")]
use super::queue::rabbitmq::Queue;

pub struct Context {
    #[cfg(feature = "postgresql")]
    pub db: Pool<ConnectionManager<PgConnection>>,
    #[cfg(feature = "mysql")]
    pub db: Pool<ConnectionManager<MysqlConnection>>,
    pub cache: Cache,
    pub queue: Queue,
    pub secret_box: SecretBox,
    pub hash_box: HashBox,
    pub jwt: Jwt,
    pub config: env::Config,
}

impl Context {
    pub fn new(cfg: &env::Config) -> Result<Self> {
        Ok(Self {
            db: Self::open_dao(&cfg.database)?,
            cache: Self::open_cache(&cfg.cache)?,
            queue: Self::open_queue(&cfg.queue)?,
            hash_box: HashBox {},
            secret_box: SecretBox::new(cfg.secret_key()?.as_slice())?,
            jwt: Jwt::new(cfg.secret_key.clone(), Algorithm::HS512),
            config: cfg.clone(),
        })
    }

    #[cfg(feature = "postgresql")]
    fn open_dao(cfg: &env::Database) -> Result<Pool<ConnectionManager<PgConnection>>> {
        cfg.postgresql.open()
    }

    #[cfg(feature = "mysql")]
    fn open_dao(cfg: &env::Database) -> Result<Pool<ConnectionManager<MysqlConnection>>> {
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
