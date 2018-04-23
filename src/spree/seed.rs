use std::io::{self, BufRead};
use std::path::PathBuf;

use log;
use validator::Validate;

use super::super::orm::Connection as Db;
use super::super::result::Result;
use super::forms::UserSignUp;
use super::models::{Role, User};

pub fn load(db: &Db, root: &PathBuf) -> Result<()> {
    let root = root.join("carmen");
    countries(db, &root)?;
    states(db, &root)?;
    zones(db, &root)?;
    administrator(db)?;
    Ok(())
}

fn administrator(db: &Db) -> Result<()> {
    // https://github.com/spree/spree/blob/master/core/db/default/spree/roles.rb
    // https://github.com/spree/spree_auth_devise/blob/master/db/default/users.rb

    if User::count(db)? == 0 {
        let stdin = io::stdin();
        println!("email:");
        let mut email = String::new();
        stdin.lock().read_line(&mut email)?;
        println!("password:");
        let mut password = String::new();
        stdin.lock().read_line(&mut password)?;
        let form = UserSignUp {
            email: s!(email.trim()),
            password: s!(password.trim()),
        };
        form.validate()?;

        log::info!("add administrator {}", &form.email);
        let user = User::sign_up(db, &form.email, &form.password)?;
        User::confirm(db, &user.id)?;
        for it in vec![Role::ROOT, Role::ADMIN, Role::MEMBER] {
            let role = Role::find_or_create(db, &s!(it))?;
            User::apply(db, &user.id, &role, 365 * 120)?;
        }
    } else {
        log::warn!("users table isn't empty")
    }

    Ok(())
}

fn countries(_db: &Db, root: &PathBuf) -> Result<()> {
    // https://github.com/lukes/ISO-3166-Countries-with-Regional-Codes
    // TODO https://github.com/carmen-ruby/carmen
    // TODO https://github.com/spree/spree/blob/master/core/db/default/spree/countries.rb
    log::info!("load countries from {:?}...", &root);

    Ok(())
}

fn states(_db: &Db, root: &PathBuf) -> Result<()> {
    // TODO https://github.com/spree/spree/blob/master/core/db/default/spree/states.rb
    log::info!("load states from {:?}...", &root);
    Ok(())
}

fn zones(_db: &Db, _root: &PathBuf) -> Result<()> {
    log::info!("add 'EU_VAT' zone");
    // TODO https://github.com/spree/spree/blob/master/core/db/default/spree/states.rb
    log::info!("add 'North America' zone");
    Ok(())
}
