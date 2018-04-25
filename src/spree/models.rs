use std::net::SocketAddr;
use std::ops::Add;
use std::ops::Deref;

use base64;
use chrono::{Duration, NaiveDate, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::{delete, insert_into, update};
use hex;
use md5::{self, Digest};
use serde_json;
use uuid::Uuid;

use super::super::orm::Connection as Db;
use super::super::result::Result;
use super::super::schema::{currencies, spree_countries, spree_role_users, spree_roles,
                           spree_states, spree_users, spree_zone_members, spree_zones};
use super::super::security::hash;

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
        let it: Currency = currencies::dsl::currencies
            .filter(currencies::dsl::iso_code.eq(iso_code))
            .first(db.deref())?;
        Ok(it)
    }
    pub fn count(db: &Db) -> Result<i64> {
        let cnt: i64 = currencies::dsl::currencies.count().get_result(db.deref())?;
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
        let it: Currency = insert_into(currencies::dsl::currencies)
            .values((
                currencies::dsl::key.eq(key),
                currencies::dsl::iso_code.eq(iso_code),
                currencies::dsl::name.eq(name),
                currencies::dsl::symbol.eq(symbol),
                currencies::dsl::alternate_symbols.eq(&serde_json::to_string(&alternate_symbols)?),
                currencies::dsl::subunit.eq(subunit),
                currencies::dsl::subunit_to_unit.eq(subunit_to_unit),
                currencies::dsl::symbol_first.eq(symbol_first),
                currencies::dsl::html_entity.eq(html_entity),
                currencies::dsl::decimal_mark.eq(decimal_mark),
                currencies::dsl::thousands_separator.eq(thousands_separator),
                currencies::dsl::iso_numeric.eq(iso_numeric),
                currencies::dsl::smallest_denomination.eq(smallest_denomination),
                currencies::dsl::updated_at.eq(&now),
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
        let it: ZoneMember = insert_into(spree_zone_members::dsl::spree_zone_members)
            .values((
                spree_zone_members::dsl::zone_id.eq(zone_id),
                spree_zone_members::dsl::zoneable_type.eq(zoneable_type),
                spree_zone_members::dsl::zoneable_id.eq(zoneable_id),
                spree_zone_members::dsl::updated_at.eq(&now),
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
    pub zone_members_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Zone {
    pub const COUNTRY: &'static str = "country";

    pub fn count(db: &Db) -> Result<i64> {
        let cnt: i64 = spree_zones::dsl::spree_zones
            .count()
            .get_result(db.deref())?;
        Ok(cnt)
    }
    pub fn add(db: &Db, name: &String, description: &String, kind: &String) -> Result<Zone> {
        let db = db.deref();
        let now = Utc::now().naive_utc();
        let it: Zone = insert_into(spree_zones::dsl::spree_zones)
            .values((
                spree_zones::dsl::name.eq(name),
                spree_zones::dsl::description.eq(description),
                spree_zones::dsl::kind.eq(kind),
                spree_zones::dsl::updated_at.eq(&now),
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
        let it: State = insert_into(spree_states::dsl::spree_states)
            .values((
                spree_states::dsl::name.eq(name),
                spree_states::dsl::abbr.eq(abbr),
                spree_states::dsl::country_id.eq(country),
                spree_states::dsl::updated_at.eq(&now),
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
        let id: i64 = spree_countries::dsl::spree_countries
            .select(spree_countries::dsl::id)
            .filter(spree_countries::dsl::iso.eq(iso))
            .first(db.deref())?;
        Ok(id)
    }
    pub fn count(db: &Db) -> Result<i64> {
        let cnt: i64 = spree_countries::dsl::spree_countries
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
        let it: Country = insert_into(spree_countries::dsl::spree_countries)
            .values((
                spree_countries::dsl::name.eq(name),
                spree_countries::dsl::iso_name.eq(iso_name),
                spree_countries::dsl::numcode.eq(numcode),
                spree_countries::dsl::iso.eq(iso),
                spree_countries::dsl::iso3.eq(iso3),
                spree_countries::dsl::states_required.eq(states_required),
                spree_countries::dsl::zipcode_required.eq(no_zipcode_iso_codes.contains(&&iso[..])),
                spree_countries::dsl::updated_at.eq(&now),
            ))
            .get_result(db.deref())?;
        Ok(it)
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Role {
    pub id: i64,
    pub name: String,
}

impl Role {
    pub const ADMIN: &'static str = "admin";
    pub const ROOT: &'static str = "root";
    pub const MEMBER: &'static str = "member";

    pub fn find_or_create(db: &Db, name: &String) -> Result<i64> {
        let db = db.deref();
        match spree_roles::dsl::spree_roles
            .select(spree_roles::dsl::id)
            .filter(spree_roles::dsl::name.eq(name))
            .first::<i64>(db)
        {
            Ok(id) => Ok(id),
            Err(_) => {
                let it: Role = insert_into(spree_roles::dsl::spree_roles)
                    .values(spree_roles::dsl::name.eq(name))
                    .get_result(db)?;
                Ok(it.id)
            }
        }
    }

    pub fn users(db: &Db, role: &i64) -> Result<Vec<(i64, String)>> {
        let db = db.deref();
        let ids = spree_role_users::dsl::spree_role_users
            .select((
                spree_role_users::dsl::user_id,
                spree_role_users::dsl::nbf,
                spree_role_users::dsl::exp,
            ))
            .filter(spree_role_users::dsl::role_id.eq(role))
            .get_results::<(i64, NaiveDate, NaiveDate)>(db)?;

        let today = Utc::now().date().naive_utc();
        let mut items = Vec::new();
        for (id, nbf, exp) in ids {
            if nbf <= today && today <= exp {
                let it = spree_users::dsl::spree_users
                    .select((spree_users::dsl::id, spree_users::dsl::email))
                    .filter(spree_users::dsl::id.eq(&id))
                    .first::<(i64, String)>(db)?;
                items.push(it);
            }
        }
        Ok(items)
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub encrypted_password: String,
    pub password_salt: Option<String>,
    pub email: String,
    pub remember_token: Option<String>,
    pub persistence_token: Option<String>,
    pub reset_password_token: Option<String>,
    pub perishable_token: Option<String>,
    pub sign_in_count: i32,
    pub failed_attempts: i32,
    pub last_request_at: Option<NaiveDateTime>,
    pub current_sign_in_at: Option<NaiveDateTime>,
    pub last_sign_in_at: Option<NaiveDateTime>,
    pub current_sign_in_ip: Option<String>,
    pub last_sign_in_ip: Option<String>,
    pub login: String,
    pub ship_address_id: Option<i64>,
    pub bill_address_id: Option<i64>,
    pub authentication_token: Option<String>,
    pub unlock_token: Option<String>,
    pub locked_at: Option<NaiveDateTime>,
    pub reset_password_sent_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub spree_api_key: Option<String>,
    pub remember_created_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub confirmation_token: Option<String>,
    pub confirmed_at: Option<NaiveDateTime>,
    pub confirmation_sent_at: Option<NaiveDateTime>,
}

impl User {
    pub fn status(&self) -> Option<String> {
        if let Some(_) = self.deleted_at {
            return Some(s!("nut.errors.user-delete"));
        }
        if let None = self.confirmed_at {
            return Some(s!("nut.errors.user-not-confirm"));
        }
        if let Some(_) = self.locked_at {
            return Some(s!("nut.errors.user-is-lock"));
        }
        None
    }
    pub fn sum_password(s: &String) -> Result<String> {
        Ok(base64::encode(&hash::sum(s.as_bytes())?))
    }
    pub fn chk_password(p: &String, s: &String) -> bool {
        if let Ok(p) = base64::decode(p) {
            return hash::verify(&p, s.as_bytes());
        }
        false
    }

    // https://en.gravatar.com/site/implement/hash/
    pub fn gravatar_logo(email: &String) -> String {
        let mut h = md5::Md5::new();
        h.input(email.to_lowercase().trim().as_bytes());
        format!(
            "https://www.gravatar.com/avatar/{}.png",
            hex::encode(h.result().as_slice())
        )
    }
    pub fn get_by_uid(db: &Db, uid: &String) -> Result<User> {
        let it = spree_users::dsl::spree_users
            .filter(spree_users::dsl::login.eq(uid))
            .first::<User>(db.deref())?;
        Ok(it)
    }
    pub fn get_by_email(db: &Db, email: &String) -> Result<User> {
        let it = spree_users::dsl::spree_users
            .filter(spree_users::dsl::email.eq(email))
            .first::<User>(db.deref())?;
        Ok(it)
    }
    pub fn count(db: &Db) -> Result<i64> {
        let cnt: i64 = spree_users::dsl::spree_users
            .count()
            .get_result(db.deref())?;
        Ok(cnt)
    }
    pub fn sign_in(
        db: &Db,
        user: &i64,
        remote: &SocketAddr,
        sign_in_count: i32,
        current_sign_in_ip: Option<String>,
        current_sign_in_at: Option<NaiveDateTime>,
    ) -> Result<()> {
        let ip = format!("{}", remote.ip());
        let it = spree_users::dsl::spree_users.filter(spree_users::dsl::id.eq(user));
        let now = Utc::now().naive_utc();
        update(it)
            .set((
                spree_users::dsl::sign_in_count.eq(sign_in_count + 1),
                spree_users::dsl::last_sign_in_ip.eq(current_sign_in_ip),
                spree_users::dsl::last_sign_in_at.eq(current_sign_in_at),
                spree_users::dsl::current_sign_in_ip.eq(ip),
                spree_users::dsl::current_sign_in_at.eq(&now),
                spree_users::dsl::updated_at.eq(&now),
            ))
            .execute(db.deref())?;
        Ok(())
    }
    pub fn sign_up(db: &Db, email: &String, password: &String) -> Result<User> {
        let now = Utc::now().naive_utc();
        let it: User = insert_into(spree_users::dsl::spree_users)
            .values((
                spree_users::dsl::email.eq(email),
                spree_users::dsl::encrypted_password.eq(&User::sum_password(password)?),
                spree_users::dsl::updated_at.eq(&now),
                spree_users::dsl::login.eq(&Uuid::new_v4().to_string()),
            ))
            .get_result(db.deref())?;
        Ok(it)
    }
    pub fn confirm(db: &Db, user: &i64) -> Result<()> {
        let it = spree_users::dsl::spree_users.filter(spree_users::dsl::id.eq(user));
        let now = Utc::now().naive_utc();
        update(it)
            .set((
                spree_users::dsl::confirmed_at.eq(&now),
                spree_users::dsl::updated_at.eq(&now),
            ))
            .execute(db.deref())?;
        Ok(())
    }

    pub fn password(db: &Db, user: &i64, password: &String) -> Result<()> {
        let it = spree_users::dsl::spree_users.filter(spree_users::dsl::id.eq(user));
        update(it)
            .set((
                spree_users::dsl::encrypted_password.eq(&User::sum_password(password)?),
                spree_users::dsl::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(db.deref())?;
        Ok(())
    }

    pub fn lock(db: &Db, user: &i64, ok: bool) -> Result<()> {
        let it = spree_users::dsl::spree_users.filter(spree_users::dsl::id.eq(user));
        let now = Utc::now().naive_utc();
        update(it)
            .set((
                spree_users::dsl::locked_at.eq(if ok { Some(now) } else { None }),
                spree_users::dsl::updated_at.eq(&now),
            ))
            .execute(db.deref())?;
        Ok(())
    }

    pub fn roles(db: &Db, user: &i64) -> Result<Vec<String>> {
        let db = db.deref();
        let ids = spree_role_users::dsl::spree_role_users
            .select((
                spree_role_users::dsl::role_id,
                spree_role_users::dsl::nbf,
                spree_role_users::dsl::exp,
            ))
            .filter(spree_role_users::dsl::user_id.eq(user))
            .get_results::<(i64, NaiveDate, NaiveDate)>(db)?;

        let today = Utc::now().date().naive_utc();
        let mut items = Vec::new();
        for (id, nbf, exp) in ids {
            if nbf <= today && today <= exp {
                let n = spree_roles::dsl::spree_roles
                    .select(spree_roles::dsl::name)
                    .filter(spree_roles::dsl::id.eq(&id))
                    .first::<String>(db)?;
                items.push(n);
            }
        }
        Ok(items)
    }

    pub fn is(db: &Db, user: &i64, role: &i64) -> bool {
        if let Ok((nbf, exp)) = spree_role_users::dsl::spree_role_users
            .select((spree_role_users::dsl::nbf, spree_role_users::dsl::exp))
            .filter(spree_role_users::dsl::user_id.eq(user))
            .filter(spree_role_users::dsl::role_id.eq(role))
            .first::<(NaiveDate, NaiveDate)>(db.deref())
        {
            let today = Utc::now().date().naive_utc();
            return nbf <= today && today <= exp;
        }
        false
    }

    pub fn apply(db: &Db, user: &i64, role: &i64, days: i64) -> Result<()> {
        let db = db.deref();
        let nbf = Utc::now();
        let exp = nbf.add(Duration::days(days));
        let nbf = nbf.date().naive_utc();
        let exp = exp.date().naive_utc();

        let now = Utc::now().naive_utc();
        match spree_role_users::dsl::spree_role_users
            .select(spree_role_users::dsl::id)
            .filter(spree_role_users::dsl::user_id.eq(user))
            .filter(spree_role_users::dsl::role_id.eq(role))
            .first::<i64>(db)
        {
            Ok(id) => {
                let it = spree_role_users::dsl::spree_role_users
                    .filter(spree_role_users::dsl::id.eq(&id));
                update(it)
                    .set((
                        spree_role_users::dsl::nbf.eq(&nbf),
                        spree_role_users::dsl::exp.eq(&exp),
                        spree_role_users::dsl::updated_at.eq(&now),
                    ))
                    .execute(db)?;
                Ok(())
            }
            Err(_) => {
                insert_into(spree_role_users::dsl::spree_role_users)
                    .values((
                        spree_role_users::dsl::role_id.eq(role),
                        spree_role_users::dsl::user_id.eq(user),
                        spree_role_users::dsl::nbf.eq(&nbf),
                        spree_role_users::dsl::exp.eq(&exp),
                        spree_role_users::dsl::updated_at.eq(&now),
                    ))
                    .execute(db)?;
                Ok(())
            }
        }
    }

    pub fn deny(db: &Db, user: &i64, role: &i64) -> Result<usize> {
        let it = spree_role_users::dsl::spree_role_users
            .filter(spree_role_users::dsl::user_id.eq(user))
            .filter(spree_role_users::dsl::role_id.eq(role));

        let num = delete(it).execute(db.deref())?;
        Ok(num)
    }
}
