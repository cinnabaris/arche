use std::collections::BTreeMap;
use std::fs::{read_dir, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use chrono::NaiveDateTime;
use diesel::connection::SimpleConnection;
use diesel::pg::types::sql_types::Timestamptz;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Text;
use log;

use super::super::super::result::Result;

const TABLE_NAME: &'static str = "__diesel_schema_migrations";
const FIRST_VERSION: &'static str = "00000000000000";

#[derive(QueryableByName)]
pub struct Model {
    #[sql_type = "Text"]
    pub version: String,
    #[sql_type = "Timestamptz"]
    pub run_on: NaiveDateTime,
}

impl<'a> super::super::Migration for super::Dao<'a> {
    fn migrate(&self) -> Result<()> {
        let files = migrations_list()?;
        let mut script = String::new();

        for (v, p) in files.iter() {
            let items = sql_query(format!(
                "SELECT version, run_on FROM {} WHERE version = '{}' LIMIT 1",
                TABLE_NAME, v
            )).load::<Model>(self.db)?;
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
        if script.is_empty() {
            return Ok(());
        }

        self.db.batch_execute(&script[..])?;
        log::info!("Done!!!");
        Ok(())
    }

    fn rollback(&self) -> Result<()> {
        let files = migrations_list()?;
        let items = sql_query(format!(
            "SELECT version, run_on FROM {} ORDER BY version DESC LIMIT 1",
            TABLE_NAME
        )).load::<Model>(self.db)?;
        if let Some(it) = items.first() {
            if let Some(down) = files.get(&it.version) {
                let mut fd = File::open(down.join("down.sql"))?;
                let mut buf = String::new();
                fd.read_to_string(&mut buf)?;

                log::info!("rollback {version}", version = it.version);
                self.db.batch_execute(&buf[..])?;
                sql_query(format!(
                    "DELETE FROM {} WHERE version = '{}'",
                    TABLE_NAME, it.version,
                )).execute(self.db)?;
                log::info!("Done!!!");
            }
        }
        Ok(())
    }

    fn versions(&self) -> Result<Vec<(String, NaiveDateTime)>> {
        let mut items = Vec::new();
        for it in sql_query(format!(
            "SELECT version, run_on FROM {} ORDER BY version ASC",
            TABLE_NAME
        )).load::<Model>(self.db)?
        {
            items.push((it.version, it.run_on));
        }

        Ok(items)
    }

    fn dump(&self) -> Result<()> {
        // TODO
        Ok(())
    }

    fn restore(&self) -> Result<()> {
        // TODO
        Ok(())
    }
}

//-----------------------------------------------------------------------------

fn migrations_dir() -> PathBuf {
    Path::new("db").join("migrations").join(super::DRIVER)
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
