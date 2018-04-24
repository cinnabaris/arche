use std::io::{self, BufRead};

use log;
use validator::Validate;

use super::super::orm::Connection as Db;
use super::super::result::Result;
use super::super::spree::models::{Role, User};
use super::c_users::FmSignUp;

pub fn administrator(db: &Db) -> Result<()> {
    if User::count(db)? > 0 {
        log::warn!("ingnore create administrator");
        return Ok(());
    }
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
        email: s!(email.trim()),
        password: s!(password.trim()),
    };
    form.validate()?;

    let user = User::sign_up(db, &form.email, &form.password)?;
    User::confirm(db, &user.id)?;
    for it in vec![Role::ROOT, Role::ADMIN, Role::MEMBER] {
        let role = Role::find_or_create(db, &s!(it))?;
        User::apply(db, &user.id, &role, 365 * 120)?;
    }

    Ok(())
}
