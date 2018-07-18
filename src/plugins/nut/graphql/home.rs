use std::ops::Deref;

use chrono::Duration;
use diesel::Connection;
use validator::Validate;

use super::super::super::super::{
    errors::{Error, Result},
    graphql::{context::Context, H},
};
use super::super::dao;

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Install {
    #[validate(length(min = "1", max = "32"))]
    pub name: String,
    #[validate(email, length(min = "2", max = "64"))]
    pub email: String,
    #[validate(length(min = "6", max = "32"))]
    pub password: String,
}

impl Install {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let db = ctx.db.deref();
        db.transaction::<_, Error, _>(|| {
            if dao::user::count(db)? > 0 {
                return Err(t!(db, &ctx.locale, "nut.errors.database-not-empty").into());
            }
            let (user, _) = dao::user::add_by_email(db, &self.name, &self.email, &self.password)?;
            l!(
                db,
                &user,
                &ctx.client_ip,
                &ctx.locale,
                "nut.logs.user.sign-up"
            )?;
            dao::user::confirm(db, &user)?;
            l!(
                db,
                &user,
                &ctx.client_ip,
                &ctx.locale,
                "nut.logs.user.confirm"
            )?;
            for it in vec![dao::role::Type::Admin, dao::role::Type::Root] {
                let ttl = Duration::weeks(1 << 12);
                dao::policy::apply(db, &user, &it, &None, &None, ttl)?;
                l!(
                    db,
                    &user,
                    &ctx.client_ip,
                    &ctx.locale,
                    "nut.logs.role.apply",
                    &Some(json!({
                            "name":format!("{}", it),
                            "type": None::<String>,
                            "id": None::<i64>,
                            "ttl": format!("{}", ttl)
                        }))
                )?;
            }
            Ok(H::new())
        })
    }
}
