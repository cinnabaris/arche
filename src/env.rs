use std::path::Path;

use base64;
use rocket;

use super::{cache, errors::Result, oauth, orm, queue, storage};

#[cfg(not(debug_assertions))]
pub fn version() -> String {
    format!("{}({})", env!("GIT_HEAD"), env!("BUILD_TIME"))
}

#[cfg(debug_assertions)]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub const NAME: &'static str = env!("CARGO_PKG_NAME");
pub const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
pub const HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");
pub const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
pub const BANNER: &'static str = r#"
_____   _____ _    _ ______
/\   |  __ \ / ____| |  | |  ____|
/  \  | |__) | |    | |__| | |__
/ /\ \ |  _  /| |    |  __  |  __|
/ ____ \| | \ \| |____| |  | | |____
/_/    \_\_|  \_\\_____|_|  |_|______|

"#;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub name: String,
    pub env: String,
    #[serde(rename = "secretkey")]
    pub secret_key: String, // 32-bits base64 encode string
    pub http: Http,
    pub oauth: oauth::Config,
    pub database: orm::Config,
    pub cache: cache::Config,
    pub queue: queue::Config,
    pub storage: storage::Config,
    pub elasticsearch: ElasticSearch,
    pub aws: Aws,
}

impl Config {
    pub fn secret_key(&self) -> Result<Vec<u8>> {
        let buf = base64::decode(&self.secret_key)?;
        return Ok(buf);
    }
    pub fn env(&self) -> rocket::config::Environment {
        match self.env.parse::<rocket::config::Environment>() {
            Ok(v) => v,
            Err(_) => rocket::config::Environment::Development,
        }
    }
    pub fn rocket(&self) -> Result<rocket::config::Config> {
        Ok(rocket::config::Config::build(self.env())
            .address("127.0.0.1")
            .port(self.http.port)
            .workers(self.http.workers)
            .log_level(match self
                .http
                .logging_level
                .parse::<rocket::config::LoggingLevel>()
            {
                Ok(v) => v,
                Err(_) => rocket::config::LoggingLevel::Debug,
            })
            .secret_key(self.secret_key.clone())
            .limits(
                rocket::config::Limits::new()
                    .limit("forms", self.http.limits)
                    .limit("json", self.http.limits),
            )
            .extra(
                "template_dir",
                match Path::new("themes")
                    .join(self.http.theme.clone())
                    .join("views")
                    .to_str()
                {
                    Some(v) => v,
                    None => "views",
                },
            )
            .finalize()?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Http {
    pub port: u16,
    pub workers: u16,
    //  one of "normal", "debug", or "critical"
    pub logging_level: String,
    pub theme: String,
    pub limits: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Aws {
    #[serde(rename = "accesskeyid")]
    pub access_key_id: String,
    #[serde(rename = "secretaccesskey")]
    pub secret_access_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ElasticSearch {
    pub hosts: Vec<String>,
}
