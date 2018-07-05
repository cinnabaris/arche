use std::fmt;

use base64;

use super::{cache, errors::Result, oauth, orm, queue, storage};

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
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
    pub env: Environment,
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Environment {
    Production,
    Staging,
    Development,
    Test,
}

impl fmt::Display for Environment {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Environment::Production => fmt.write_str("production"),
            Environment::Staging => fmt.write_str("staging"),
            Environment::Development => fmt.write_str("development"),
            Environment::Test => fmt.write_str("test"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Http {
    pub port: u16,
    pub theme: String,
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
