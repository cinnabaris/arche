use juniper;
use rocket::http::Status;

use super::super::{
    env::Config, errors::Result, orm::PooledConnection as Db, plugins::nut::dao::Role,
    utils::Encryptor,
};

pub struct Context {
    pub home: String,
    pub locale: String,
    pub token: Option<String>,
    pub client_ip: String,
    pub encryptor: Encryptor,
    pub db: Db,
    pub config: Config,
}

impl juniper::Context for Context {}

impl Context {
    pub fn current_user(&self) -> Result<i64> {
        if let Some(ref _token) = self.token {
            // TODO
            return Err(Status::Unauthorized.reason.into());
        }

        Err(Status::NonAuthoritativeInformation.reason.into())
    }
    pub fn admin(&self) -> Result<i64> {
        self.must(&Role::Admin.to_string(), &None, &None)
    }
    pub fn must(&self, _name: &String, _type_: &Option<String>, _id: &Option<i64>) -> Result<i64> {
        let _id = self.current_user()?;
        // TODO
        Err(Status::Forbidden.reason.into())
    }
    pub fn client_ip(&self) -> Result<String> {
        Ok("ip".to_string())
    }
}
