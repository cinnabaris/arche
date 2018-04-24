use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

use log;
use serde_json;

use super::super::i18n::Locale;
use super::super::orm::Connection as Db;
use super::super::result::Result;
use super::models::{Country, Currency, State, Zone, ZoneMember};

pub fn load(db: &Db, root: &PathBuf) -> Result<()> {
    regions(db, root)?;
    zones(db)?;
    currencies(db, root)?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CurrencyItem {
    pub key: String,
    pub iso_code: String,
    pub name: String,
    pub symbol: Option<String>,
    pub alternate_symbols: Option<Vec<String>>,
    pub subunit: Option<String>,
    pub subunit_to_unit: i32,
    pub symbol_first: bool,
    pub html_entity: Option<String>,
    pub decimal_mark: String,
    pub thousands_separator: String,
    pub iso_numeric: Option<i32>,
    pub smallest_denomination: Option<i32>,
}

fn currencies(db: &Db, root: &PathBuf) -> Result<()> {
    if Currency::count(db)? > 0 {
        log::warn!("ingnore import currencies");
        return Ok(());
    }

    let root = root.join("money");
    // https://www.iso.org/iso-4217-currency-codes.html
    let file = root.join("currencies.json");
    log::info!("load currencies data from {:?}...", &file);
    let currencies: Vec<CurrencyItem> = serde_json::from_reader(File::open(file)?)?;

    let mut total: isize = 0;

    for it in currencies {
        Currency::add(
            db,
            &it.key,
            &it.iso_code,
            &it.name,
            &it.symbol,
            &it.alternate_symbols,
            &it.subunit,
            &it.subunit_to_unit,
            &it.symbol_first,
            &it.html_entity,
            &it.decimal_mark,
            &it.thousands_separator,
            &it.iso_numeric,
            &it.smallest_denomination,
        )?;

        total = total + 1;
    }
    log::info!("find {} currencies", total);

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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LocaleItem {
    code: String,
    message: String,
}

fn regions(db: &Db, root: &PathBuf) -> Result<()> {
    if Country::count(db)? > 0 {
        log::warn!("ingnore import countries,states,zones");
        return Ok(());
    }

    let root = root.join("carmen");
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

    let mut langs = HashMap::new();
    langs.insert("en-US", "en");
    langs.insert("zh-Hans", "cn");
    langs.insert("zh-Hant", "tw");
    for (l, n) in langs {
        let mut file = root.join(n);
        file.set_extension("json");
        log::info!("load locales from {:?}", file);
        let items: Vec<LocaleItem> = serde_json::from_reader(File::open(file)?)?;
        let mut total = 0;
        for it in items {
            Locale::set(db, &s!(l), &it.code, &it.message)?;
            total = total + 1;
        }
        log::info!("insert {}", total);
    }

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
