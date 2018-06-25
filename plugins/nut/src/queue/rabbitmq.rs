use std::time::{SystemTime, UNIX_EPOCH};

use amqp::{self, Basic, Options};
use log;
use uuid::Uuid;

use super::super::errors::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    #[serde(rename = "virtual")]
    pub _virtual: String,
}

impl Config {
    pub fn options(&self) -> Options {
        Options {
            host: self.host.clone(),
            port: self.port,
            login: self.user.clone(),
            password: self.password.clone(),
            vhost: self._virtual.clone(),
            ..Default::default()
        }
    }
}

pub struct Queue {
    queue: String,
    cfg: Config,
}

impl Queue {
    pub fn new(queue: String, cfg: Config) -> Self {
        Self {
            queue: queue,
            cfg: cfg,
        }
    }
    fn open<F>(&self, f: F) -> Result<()>
    where
        F: Fn(&mut amqp::Channel) -> Result<()>,
    {
        let mut ss = amqp::Session::new(self.cfg.options())?;
        let mut ch = ss.open_channel(1)?;
        ch.queue_declare(
            &self.queue[..],
            false, // passive,
            true,  // durable
            false, // exclusive
            false, // auto_delete
            false, // nowait
            amqp::Table::new(),
        )?;

        f(&mut ch)?;

        ch.close(200, "Bye")?;
        ss.close(200, "Good Bye");

        Ok(())
    }
}

impl super::Provider for Queue {
    fn publish(
        &self,
        _type: &String,
        content_type: &String,
        priority: u8,
        payload: &[u8],
    ) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        log::info!("push task into queue {}@{}", id, self.queue);
        self.open(|ch| {
            ch.basic_publish(
                "",
                &self.queue[..],
                true,
                false,
                amqp::protocol::basic::BasicProperties {
                    content_type: Some(content_type.to_string()),
                    _type: Some(_type.to_string()),
                    priority: Some(priority),
                    timestamp: Some(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()),
                    message_id: Some(id.clone()),
                    ..Default::default()
                },
                payload.to_vec(),
            )?;
            return Ok(());
        })
    }
}
