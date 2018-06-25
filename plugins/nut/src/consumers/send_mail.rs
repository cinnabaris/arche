use std::collections::BTreeMap;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

use lettre::{
    smtp::authentication::{Credentials, Mechanism},
    EmailTransport, SmtpTransport,
};
use lettre_email::EmailBuilder;
use log;
use mime;
use serde_json;

use super::super::{context::Context, errors::Result, orm::Dao, queue, settings};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mail {
    pub to: String,
    pub subject: String,
    pub body: String,
    pub attachments: BTreeMap<PathBuf, String>,
}

pub const NAME: &'static str = "send-mail";

pub struct Consumer {
    ctx: Arc<Context>,
}

impl Consumer {
    pub fn new(ctx: Arc<Context>) -> Self {
        Self { ctx: ctx }
    }
}

impl queue::consumer::Handler for Consumer {
    fn handle(
        &self,
        _id: &String,
        _type: &String,
        _content_type: &String,
        _priority: u8,
        payload: &[u8],
    ) -> Result<()> {
        let it: Mail = serde_json::from_slice(payload)?;
        if !self.ctx.config.is_prod() {
            log::debug!("send email to {}: {}\n{}", it.to, it.subject, it.body);
            return Ok(());
        }
        let db = self.ctx.db.get()?;
        let db = Dao::new(db.deref());
        let cfg: Config = settings::get(&db, &self.ctx.encryptor, &String::from("site.smtp"))?;

        let mut email = EmailBuilder::new()
            .to(it.to)
            .from(&cfg.user[..])
            .subject(it.subject)
            .html(it.body);
        for (file, name) in it.attachments {
            email.set_attachment(file.as_path(), Some(&name[..]), &mime::TEXT_PLAIN)?;
        }
        let email = email.build()?;

        let mut mailer = SmtpTransport::simple_builder(&cfg.host)?
            // .hello_name(ClientId::Domain(sys_info::hostname()?))
            .credentials(Credentials::new(cfg.user, cfg.password))
            .smtp_utf8(true)
            .authentication_mechanism(Mechanism::Plain)
            .build();

        mailer.send(&email)?;

        Ok(())
    }
}
