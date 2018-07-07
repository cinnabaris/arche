use std::ops::Deref;
use std::sync::Arc;

use juniper;
use rocket::http::Status;

use super::super::{
    context::Context as AppContext,
    errors::Result,
    orm::PooledConnection as Db,
    plugins::nut::dao::{policy as policy_dao, role::Type as RoleType, user as user_dao},
};

pub struct Context {
    pub home: String,
    pub locale: String,
    pub token: Option<String>,
    pub client_ip: String,
    pub app: Arc<AppContext>,
    pub db: Db,
}

impl juniper::Context for Context {}

impl Context {
    pub fn current_user(&self) -> Result<i64> {
        if let Some(ref token) = self.token {
            let payload = self.app.jwt.parse(token)?;
            if let Some(uid) = payload.get(super::UID.to_string()) {
                if let Some(uid) = uid.as_str() {
                    return user_dao::get_by_uid(self.db.deref(), &uid.to_string());
                }
            }
            return Err(Status::Unauthorized.reason.into());
        }
        Err(Status::NonAuthoritativeInformation.reason.into())
    }
    pub fn admin(&self) -> Result<i64> {
        self.must(&RoleType::Admin, &None, &None)
    }
    pub fn must(
        &self,
        role_type: &RoleType,
        resource_type: &Option<String>,
        resource_id: &Option<i64>,
    ) -> Result<i64> {
        let user = self.current_user()?;
        if policy_dao::can(
            self.db.deref(),
            &user,
            role_type,
            resource_type,
            resource_id,
        ) {
            return Ok(user);
        }
        Err(Status::Forbidden.reason.into())
    }
    pub fn client_ip(&self) -> Result<String> {
        Ok("ip".to_string())
    }
}
