use std::ops::Deref;

use chrono::{NaiveDateTime, Utc};
use diesel::insert_into;
use diesel::prelude::*;
use serde_json;

use super::super::orm::Connection as Db;
use super::super::result::Result;
use super::super::schema::{mall_countries, mall_currencies, mall_states, mall_zone_members,
                           mall_zones};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Currency {
    pub id: i64,
    pub key: String,
    pub iso_code: String,
    pub name: String,
    pub symbol: Option<String>,
    pub alternate_symbols: String,
    pub subunit: Option<String>,
    pub subunit_to_unit: i32,
    pub symbol_first: bool,
    pub html_entity: Option<String>,
    pub decimal_mark: String,
    pub thousands_separator: String,
    pub iso_numeric: Option<i32>,
    pub smallest_denomination: Option<i32>,
    pub updated_at: NaiveDateTime,
}

impl Currency {
    pub fn alternate_symbols(&self) -> Result<Vec<String>> {
        let items: Vec<String> = serde_json::from_slice(self.alternate_symbols.as_bytes())?;
        Ok(items)
    }
    pub fn get_by_iso_code(db: &Db, iso_code: &String) -> Result<Currency> {
        let it: Currency = mall_currencies::dsl::mall_currencies
            .filter(mall_currencies::dsl::iso_code.eq(iso_code))
            .first(db.deref())?;
        Ok(it)
    }
    pub fn count(db: &Db) -> Result<i64> {
        let cnt: i64 = mall_currencies::dsl::mall_currencies
            .count()
            .get_result(db.deref())?;
        Ok(cnt)
    }
    pub fn add(
        db: &Db,
        key: &String,
        iso_code: &String,
        name: &String,
        symbol: &Option<String>,
        alternate_symbols: &Option<Vec<String>>,
        subunit: &Option<String>,
        subunit_to_unit: &i32,
        symbol_first: &bool,
        html_entity: &Option<String>,
        decimal_mark: &String,
        thousands_separator: &String,
        iso_numeric: &Option<i32>,
        smallest_denomination: &Option<i32>,
    ) -> Result<Currency> {
        let db = db.deref();
        let now = Utc::now().naive_utc();
        let alternate_symbols = match alternate_symbols {
            Some(v) => json!(v),
            None => json!({}),
        };
        let it: Currency = insert_into(mall_currencies::dsl::mall_currencies)
            .values((
                mall_currencies::dsl::key.eq(key),
                mall_currencies::dsl::iso_code.eq(iso_code),
                mall_currencies::dsl::name.eq(name),
                mall_currencies::dsl::symbol.eq(symbol),
                mall_currencies::dsl::alternate_symbols
                    .eq(&serde_json::to_string(&alternate_symbols)?),
                mall_currencies::dsl::subunit.eq(subunit),
                mall_currencies::dsl::subunit_to_unit.eq(subunit_to_unit),
                mall_currencies::dsl::symbol_first.eq(symbol_first),
                mall_currencies::dsl::html_entity.eq(html_entity),
                mall_currencies::dsl::decimal_mark.eq(decimal_mark),
                mall_currencies::dsl::thousands_separator.eq(thousands_separator),
                mall_currencies::dsl::iso_numeric.eq(iso_numeric),
                mall_currencies::dsl::smallest_denomination.eq(smallest_denomination),
                mall_currencies::dsl::updated_at.eq(&now),
            ))
            .get_result(db)?;
        Ok(it)
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct ZoneMember {
    pub id: i64,
    pub zone_id: i64,
    pub zoneable_type: String,
    pub zoneable_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl ZoneMember {
    pub fn add(
        db: &Db,
        zone_id: &i64,
        zoneable_type: &String,
        zoneable_id: &i64,
    ) -> Result<ZoneMember> {
        let db = db.deref();
        let now = Utc::now().naive_utc();
        let it: ZoneMember = insert_into(mall_zone_members::dsl::mall_zone_members)
            .values((
                mall_zone_members::dsl::zone_id.eq(zone_id),
                mall_zone_members::dsl::zoneable_type.eq(zoneable_type),
                mall_zone_members::dsl::zoneable_id.eq(zoneable_id),
                mall_zone_members::dsl::updated_at.eq(&now),
            ))
            .get_result(db)?;
        Ok(it)
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Zone {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub kind: String,
    pub default_tax: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Zone {
    pub const COUNTRY: &'static str = "country";

    pub fn count(db: &Db) -> Result<i64> {
        let cnt: i64 = mall_zones::dsl::mall_zones.count().get_result(db.deref())?;
        Ok(cnt)
    }
    pub fn add(db: &Db, name: &String, description: &String, kind: &String) -> Result<Zone> {
        let db = db.deref();
        let now = Utc::now().naive_utc();
        let it: Zone = insert_into(mall_zones::dsl::mall_zones)
            .values((
                mall_zones::dsl::name.eq(name),
                mall_zones::dsl::description.eq(description),
                mall_zones::dsl::kind.eq(kind),
                mall_zones::dsl::updated_at.eq(&now),
            ))
            .get_result(db)?;
        Ok(it)
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct State {
    pub id: i64,
    pub name: String,
    pub abbr: String,
    pub country_id: i64,
    pub updated_at: NaiveDateTime,
}

impl State {
    pub fn add(db: &Db, country: &i64, name: &String, abbr: &String) -> Result<State> {
        let now = Utc::now().naive_utc();
        let it: State = insert_into(mall_states::dsl::mall_states)
            .values((
                mall_states::dsl::name.eq(name),
                mall_states::dsl::abbr.eq(abbr),
                mall_states::dsl::country_id.eq(country),
                mall_states::dsl::updated_at.eq(&now),
            ))
            .get_result(db.deref())?;
        Ok(it)
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Country {
    pub id: i64,
    pub name: String,
    pub iso_name: String,
    pub numcode: i32,
    pub iso: String,
    pub iso3: String,
    pub states_required: bool,
    pub zipcode_required: bool,
    pub updated_at: NaiveDateTime,
}

impl Country {
    pub fn get_id_by_iso(db: &Db, iso: &String) -> Result<i64> {
        let id: i64 = mall_countries::dsl::mall_countries
            .select(mall_countries::dsl::id)
            .filter(mall_countries::dsl::iso.eq(iso))
            .first(db.deref())?;
        Ok(id)
    }
    pub fn count(db: &Db) -> Result<i64> {
        let cnt: i64 = mall_countries::dsl::mall_countries
            .count()
            .get_result(db.deref())?;
        Ok(cnt)
    }
    pub fn add(
        db: &Db,
        name: &String,
        iso_name: &String,
        numcode: &i32,
        iso: &String,
        iso3: &String,
        states_required: &bool,
    ) -> Result<Country> {
        let now = Utc::now().naive_utc();
        let no_zipcode_iso_codes = vec![
            "AO", "AG", "AW", "BS", "BZ", "BJ", "BM", "BO", "BW", "BF", "BI", "CM", "CF", "KM",
            "CG", "CD", "CK", "CUW", "CI", "DJ", "DM", "GQ", "ER", "FJ", "TF", "GAB", "GM", "GH",
            "GD", "GN", "GY", "HK", "IE", "KI", "KP", "LY", "MO", "MW", "ML", "MR", "NR", "AN",
            "NU", "KP", "PA", "QA", "RW", "KN", "LC", "ST", "SC", "SL", "SB", "SO", "SR", "SY",
            "TZ", "TL", "TK", "TG", "TO", "TV", "UG", "AE", "VU", "YE", "ZW",
        ];
        let it: Country = insert_into(mall_countries::dsl::mall_countries)
            .values((
                mall_countries::dsl::name.eq(name),
                mall_countries::dsl::iso_name.eq(iso_name),
                mall_countries::dsl::numcode.eq(numcode),
                mall_countries::dsl::iso.eq(iso),
                mall_countries::dsl::iso3.eq(iso3),
                mall_countries::dsl::states_required.eq(states_required),
                mall_countries::dsl::zipcode_required.eq(no_zipcode_iso_codes.contains(&&iso[..])),
                mall_countries::dsl::updated_at.eq(&now),
            ))
            .get_result(db.deref())?;
        Ok(it)
    }
}
