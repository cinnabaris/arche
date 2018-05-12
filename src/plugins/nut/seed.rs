use std::io::{self, BufRead};

use chrono::Duration;
use diesel::Connection;
use log;
use validator::Validate;

use super::super::super::{
    orm::Connection as Db, result::{Error, Result},
};
use super::{
    models::{Policy, Role, User}, schema::users::FmSignUp,
};

pub fn administrator(db: &Db) -> Result<()> {
    if User::count(db)? > 0 {
        log::warn!("ingnore create administrator");
        return Ok(());
    }
    db.transaction::<_, Error, _>(|| {
        // https://github.com/spree/spree/blob/master/core/db/default/spree/roles.rb
        // https://github.com/spree/spree_auth_devise/blob/master/db/default/users.rb
        log::info!("create an administrator");
        let stdin = io::stdin();
        println!("email:");
        let mut email = String::new();
        stdin.lock().read_line(&mut email)?;
        println!("password:");
        let mut password = String::new();
        stdin.lock().read_line(&mut password)?;
        let form = FmSignUp {
            name: s!("Administrator"),
            email: s!(email.trim()),
            password: s!(password.trim()),
        };
        form.validate()?;

        let user = User::sign_up(db, &form.name, &form.email, &form.password)?;
        User::set_confirmed(db, &user.id)?;
        for it in vec![Role::ROOT, Role::ADMIN, Role::MEMBER] {
            let role = Role::get(db, &s!(it), &None, &None)?;
            Policy::apply(db, &user.id, &role, Duration::weeks(5 * 1024))?;
        }

        Ok(())
    })
}
