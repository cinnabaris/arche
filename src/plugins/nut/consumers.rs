use std::collections::BTreeMap;
use std::ops::Deref;
use std::path::PathBuf;

use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::extension::ClientId;
use lettre::{EmailTransport, SmtpTransport};
use lettre_email::EmailBuilder;
use log;
use mime;
use serde_json;
use sys_info;

use super::super::super::{context::Context, dao::Dao, result::Result, settings};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Smtp {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Email {
    pub to: String,
    pub subject: String,
    pub body: String,
    pub attachments: BTreeMap<PathBuf, String>,
}

pub const SEND_EMAIL: &'static str = "send-email";

pub trait SendEmail {
    fn send_email(&self, &[u8]) -> Result<()>;
}

impl SendEmail for Context {
    fn send_email(&self, payload: &[u8]) -> Result<()> {
        let it: Email = serde_json::from_slice(payload)?;
        if !self.config.is_prod() {
            log::debug!("send email to {}: {}\n{}", it.to, it.subject, it.body);
            return Ok(());
        }
        let db = self.db.get()?;
        let db = Dao::new(db.deref());
        let smtp: Smtp = settings::get(&db, &self.secret_box, &s!("site.smtp"))?;

        let mut email = EmailBuilder::new()
            .to(it.to)
            .from(&smtp.user[..])
            .subject(it.subject)
            .html(it.body);
        for (file, name) in it.attachments {
            email.set_attachment(file.as_path(), Some(&name[..]), &mime::TEXT_PLAIN)?;
        }
        let email = email.build()?;

        let mut mailer = SmtpTransport::simple_builder(&smtp.host)?
            .hello_name(ClientId::Domain(sys_info::hostname()?))
            .credentials(Credentials::new(smtp.user, smtp.password))
            .smtp_utf8(true)
            .authentication_mechanism(Mechanism::Plain)
            .build();

        mailer.send(&email)?;

        Ok(())
    }
}
