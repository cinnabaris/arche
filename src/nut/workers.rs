use std::collections::BTreeMap;
use std::default::Default;
use std::path::PathBuf;

use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::extension::ClientId;
use lettre::{EmailTransport, SmtpTransport};
use lettre_email::EmailBuilder;
use log;
use mime;
use serde_json;
use sys_info;

use super::super::queue::Consumer;
use super::super::result::Result;

pub const SEND_EMAIL: &'static str = "send-email";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Smtp {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Email {
    to: String,
    subject: String,
    body: String,
    attachments: BTreeMap<PathBuf, String>,
}

pub trait SendEmail {
    fn send_email(&self, payload: &[u8], perform: bool) -> Result<()>;
}

impl SendEmail for Consumer {
    fn send_email(&self, payload: &[u8], perform: bool) -> Result<()> {
        let it: Email = try!(serde_json::from_slice(payload));
        if !perform {
            log::debug!("send email to {}: {}\n{}", it.to, it.subject, it.body);
            return Ok(());
        }
        // let smtp = try!(Setting::get(db, sec, &"site.smtp".to_string()));
        // let smtp: Smtp = try!(serde_json::from_slice(&smtp));
        let smtp: Smtp = Default::default();

        let mut email = EmailBuilder::new()
            .to(it.to)
            .from(&smtp.user[..])
            .subject(it.subject)
            .html(it.body);
        for (file, name) in it.attachments {
            try!(email.set_attachment(file.as_path(), Some(&name[..]), &mime::TEXT_PLAIN));
        }
        let email = try!(email.build());

        let mut mailer = try!(SmtpTransport::simple_builder(&smtp.host))
            .hello_name(ClientId::Domain(sys_info::hostname()?))
            .credentials(Credentials::new(smtp.user, smtp.password))
            .smtp_utf8(true)
            .authentication_mechanism(Mechanism::Plain)
            .build();

        try!(mailer.send(&email));

        Ok(())
    }
}

// pub fn send_email<MQ: Queue>(mq: &MQ, to: String, subject: String, body: String) -> Result<()> {
//     mq.publish(
//         &SEND_EMAIL.to_string(),
//         &mime::APPLICATION_JSON.as_ref().to_string(),
//         8,
//         try!(serde_json::to_vec(&EmailJob {
//             to: to,
//             subject: subject,
//             body: body,
//             attachments: BTreeMap::new(),
//         },))
//             .as_slice(),
//     )
// }
//
// pub fn do_send_email<D: postgres::GenericConnection, S: Security>(
//     db: &D,
//     sec: &S,
//     payload: &[u8],
//     perform: bool,
// ) -> Result<()> {

//

// }
