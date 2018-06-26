use std::default::Default;

use base64;
use hyper::header::{Authorization, Bearer, ContentType, Header};
use rocket::{
    config::{Environment, Limits},
    http::Method,
};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};

use super::{cache, errors::Result, oauth, orm, queue, storage};

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
    pub languages: Vec<String>,
    #[serde(rename = "secretkey")]
    pub secret_key: String, // 32-bits base64 encode string
    pub workers: u16,
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
    pub fn env(&self) -> Result<Environment> {
        match self.env.parse::<Environment>() {
            Ok(v) => Ok(v),
            Err(_) => Err("bad env".into()),
        }
    }

    pub fn is_prod(&self) -> bool {
        if let Ok(v) = self.env() {
            return v.is_prod();
        }
        return false;
    }

    pub fn secret_key(&self) -> Result<Vec<u8>> {
        let buf = base64::decode(&self.secret_key)?;
        return Ok(buf);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Http {
    pub port: u16,
    pub theme: String,
    pub limits: u64,
    pub origins: Vec<String>,
}

impl Http {
    pub fn limits(&self) -> Limits {
        Limits::new()
            .limit("forms", self.limits)
            .limit("json", self.limits)
    }

    pub fn cors(&self) -> Cors {
        let origins: Vec<&str> = self.origins.iter().map(AsRef::as_ref).collect();
        let (allowed_origins, _failed_origins) = AllowedOrigins::some(origins.as_slice());
        Cors {
            allowed_origins: allowed_origins,
            allowed_methods: vec![
                Method::Get,
                Method::Post,
                Method::Patch,
                Method::Put,
                Method::Delete,
            ].into_iter()
                .map(From::from)
                .collect(),
            allowed_headers: AllowedHeaders::some(&[
                Authorization::<Bearer>::header_name(),
                ContentType::header_name(),
                "X-Requested-With",
            ]),
            allow_credentials: true,
            ..Default::default()
        }
    }
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
