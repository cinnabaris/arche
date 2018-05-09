use std::collections::BTreeMap;
use std::fs::{read_dir, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use chrono::NaiveDateTime;
use diesel::connection::Connection;
use diesel::connection::SimpleConnection;
#[cfg(feature = "postgresql")]
use diesel::pg::types::sql_types::Timestamptz;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Text;
use log;

use super::orm::Connection as Db;
use super::result::{Error, Result};

#[cfg(feature = "mysql")]
const DRIVER: &'static str = "mysql";
#[cfg(feature = "postgresql")]
const DRIVER: &'static str = "postgresql";

const TABLE_NAME: &'static str = "__diesel_schema_migrations";
const FIRST_VERSION: &'static str = "00000000000000";

#[derive(QueryableByName)]
pub struct Model {
    #[sql_type = "Text"]
    pub version: String,
    #[sql_type = "Timestamptz"]
    pub run_on: NaiveDateTime,
}

pub fn migrate(db: &Db) -> Result<()> {
    let files = migrations_list()?;
    let mut script = String::new();

    for (v, p) in files.iter() {
        let items = sql_query(format!(
            "SELECT version, run_on FROM {} WHERE version = '{}' LIMIT 1",
            TABLE_NAME, v
        )).load::<Model>(db)?;
        let act = if items.is_empty() {
            let mut fd = File::open(p.join("up.sql"))?;
            let mut buf = String::new();
            fd.read_to_string(&mut buf)?;
            script += &buf;
            script += &format!("INSERT INTO {}(version) VALUES('{}');", TABLE_NAME, v);
            "migrate"
        } else {
            "ingnore"
        };

        log::info!("{action} migration {version}", action = act, version = v);
    }

    db.transaction::<_, Error, _>(|| {
        db.batch_execute(&script[..])?;
        log::info!("Done!!!");
        Ok(())
    })
}

pub fn rollback(db: &Db) -> Result<()> {
    let files = migrations_list()?;
    let items = sql_query(format!(
        "SELECT version, run_on FROM {} ORDER BY version DESC LIMIT 1",
        TABLE_NAME
    )).load::<Model>(db)?;
    if let Some(it) = items.first() {
        if let Some(down) = files.get(&it.version) {
            let mut fd = File::open(down.join("down.sql"))?;
            let mut buf = String::new();
            fd.read_to_string(&mut buf)?;
            return db.transaction::<_, Error, _>(|| {
                log::info!("rollback {version}", version = it.version);
                db.batch_execute(&buf[..])?;
                sql_query(format!(
                    "DELETE FROM {} WHERE version = '{}'",
                    TABLE_NAME, it.version,
                )).execute(db)?;
                log::info!("Done!!!");
                Ok(())
            });
        }
    }
    Ok(())
}

pub fn versions(db: &Db) -> Result<Vec<Model>> {
    Ok(sql_query(format!(
        "SELECT version, run_on FROM {} ORDER BY version ASC",
        TABLE_NAME
    )).load::<Model>(db)?)
}

pub fn dump(_db: &Db) -> Result<()> {
    // TODO
    Ok(())
}

pub fn restore(_db: &Db) -> Result<()> {
    // TODO
    Ok(())
}

//-----------------------------------------------------------------------------

fn migrations_dir() -> PathBuf {
    Path::new("db").join("migrations").join(DRIVER)
}

fn migrations_list() -> Result<BTreeMap<String, PathBuf>> {
    let mut items = BTreeMap::new();
    for entry in read_dir(migrations_dir())? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name() {
                if let Some(name) = name.to_str() {
                    if let Some(i) = name.find('_') {
                        let name = &name[..i];
                        let version = if name == FIRST_VERSION {
                            s!(name)
                        } else {
                            NaiveDateTime::parse_from_str(name, "%Y-%m-%d-%H%M%S")?
                                .format("%Y%m%d%H%M%S")
                                .to_string()
                        };
                        items.insert(version, path.clone());
                    }
                }
            }
        }
    }
    Ok(items)
}
