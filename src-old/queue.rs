use super::nut::workers::{SendEmail, SEND_EMAIL};
use super::orm::{Connection as Db, Pool as DbPool};
use super::security;

pub type Queue = RabbitMQ;

//-----------------------------------------------------------------------------

pub struct Consumer {
    name: String,
    env: Environment,
    db: DbPool,
    encryptor: security::Encryptor,
}

impl Consumer {
    pub fn new(name: String, env: Environment, db: DbPool, enc: security::Encryptor) -> Self {
        Self {
            name: name,
            env: env,
            db: db,
            encryptor: enc,
        }
    }

    fn run(
        &self,
        _type: &String,
        _id: &String,
        _content_type: &String,
        payload: &[u8],
    ) -> Result<()> {
        let db = Db(self.db.get()?);
        match &_type[..] {
            SEND_EMAIL => self.send_email(&db, &self.encryptor, payload, self.env.is_prod()),
            _ => Err(Error::WithDescription(format!("bad task type {}", _type))),
        }
    }
}

//-----------------------------------------------------------------------------

pub trait Provider: Send + Sync {}

pub struct RabbitMQ {
    url: String,
    queue: String,
}
