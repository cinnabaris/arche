use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

use log;
use serde_json;
use validator::Validate;

use super::super::orm::Connection as Db;
use super::super::result::Result;
use super::forms::UserSignUp;
use super::models::{Country, Role, State, User, Zone, ZoneMember};

pub fn load(db: &Db, root: &PathBuf) -> Result<()> {
    let root = root.join("carmen");
    regions(db, &root)?;
    zones(db)?;
    currencies(db, &root)?;
    administrator(db)?;
    Ok(())
}

fn administrator(db: &Db) -> Result<()> {
    if User::count(db)? > 0 {
        log::warn!("ingnore create administrator");
        return Ok(());
    }
    // https://github.com/spree/spree/blob/master/core/db/default/spree/roles.rb
    // https://github.com/spree/spree_auth_devise/blob/master/db/default/users.rb

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

    Ok(())
}

fn currencies(_db: &Db, _root: &PathBuf) -> Result<()> {
    // https://www.iso.org/iso-4217-currency-codes.html
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CountryItem {
    name: String,
    iso_name: String,
    numcode: i32,
    iso: String,
    iso3: String,
    states_required: bool,
    states: Vec<StateItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct StateItem {
    name: String,
    abbr: String,
}

fn regions(db: &Db, root: &PathBuf) -> Result<()> {
    if Country::count(db)? > 0 {
        log::warn!("ingnore import countries,states,zones");
        return Ok(());
    }
    // https://github.com/lukes/ISO-3166-Countries-with-Regional-Codes
    // https://github.com/carmen-ruby/carmen
    // https://github.com/spree/spree/blob/master/core/db/default/spree/countries.rb
    // https://github.com/spree/spree/blob/master/core/db/default/spree/states.rb
    let file = root.join("countries.json");
    log::info!("load regions data from {:?}...", &file);
    let countries: Vec<CountryItem> = serde_json::from_reader(File::open(file)?)?;

    let mut ct: isize = 0;
    let mut st: isize = 0;

    for c in countries {
        let it = Country::add(
            db,
            &c.name,
            &c.iso_name,
            &c.numcode,
            &c.iso,
            &c.iso3,
            &c.states_required,
        )?;
        for s in c.states {
            State::add(db, &it.id, &s.name, &s.abbr)?;
            st = st + 1;
        }
        ct = ct + 1;
    }
    log::info!("find {} countries, {} states", ct, st);

    Ok(())
}

fn zones(db: &Db) -> Result<()> {
    if Zone::count(db)? > 0 {
        log::warn!("ingnore import zones");
        return Ok(());
    }

    // https://github.com/spree/spree/blob/master/core/db/default/spree/zones.rb
    add_zone(
        db,
        &s!("EU_VAT"),
        &s!("Countries that make up the EU VAT zone."),
        &vec![
            "PL", "FI", "PT", "RO", "DE", "FR", "SK", "HU", "SI", "IE", "AT", "ES", "IT", "BE",
            "SE", "LV", "BG", "GB", "LT", "CY", "LU", "MT", "DK", "NL", "EE", "HR", "CZ", "GR",
        ],
    )?;
    add_zone(
        db,
        &s!("North America"),
        &s!("USA + Canada"),
        &vec!["US", "CA"],
    )?;
    Ok(())
}

fn add_zone(
    db: &Db,
    name: &String,
    description: &String,
    countries: &Vec<&'static str>,
) -> Result<()> {
    log::info!("add {:?} zone", name);
    let kind = s!(Zone::COUNTRY);
    let z = Zone::add(db, name, description, &kind)?;
    for c in countries {
        ZoneMember::add(db, &z.id, &kind, &Country::get_id_by_iso(db, &s!(c))?)?;
    }
    Ok(())
}
