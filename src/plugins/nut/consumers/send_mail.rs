use std::collections::BTreeMap;
use std::ops::Deref;
use std::path::PathBuf;

use lettre::{
    smtp::authentication::{Credentials, Mechanism},
    EmailTransport, SmtpTransport,
};
use lettre_email::EmailBuilder;
use log;
use mime;
use rocket::config::Environment;
use serde_json;

use super::super::super::super::{context::Context, errors::Result, queue, settings};

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

pub struct Consumer {}

impl queue::Consumer for Consumer {
    fn consume(
        &self,
        ctx: &Context,
        _id: &String,
        _content_type: &String,
        _priority: u8,
        payload: &[u8],
    ) -> Result<()> {
        let it: Mail = serde_json::from_slice(payload)?;
        match ctx.config.env() {
            Environment::Production => {
                let db = ctx.db.get()?;
                let db = db.deref();
                let cfg: Config = settings::get(db, &ctx.encryptor, &String::from("site.smtp"))?;

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
            _ => {
                log::debug!("send email to {}: {}\n{}", it.to, it.subject, it.body);
                Ok(())
            }
        }
    }
}
