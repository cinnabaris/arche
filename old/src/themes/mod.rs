pub mod hugo;
pub mod jekyll;

use std::fs::OpenOptions;
use std::io::Write;
use std::ops::Deref;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;

use chrono::NaiveDateTime;
use log;

use super::{
    dao::{Connection, Dao}, result::Result,
};

// https://en.wikipedia.org/wiki/Tz_database
pub fn generate<T: Template>(
    src: &PathBuf,
    dst: &PathBuf,
    db: &Connection,
    tpl: &T,
    queries: Vec<Box<Query>>,
) -> Result<()> {
    {
        let db = db.deref();
        let db = Dao::new(db);
        tpl.generate_config(dst, &db)?;
    }

    for qry in queries.iter() {
        let ids = qry.ids(db)?;
        for id in ids.iter() {
            let pg = qry.page(db, id)?;
            tpl.generate_page(dst, &pg)?;
        }
    }

    let file = dst.join("build.sh");
    log::info!("generate file {}", file.display());
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o700)
        .open(file)?;
    file.write_all(tpl.build_script(src, dst)?.as_bytes())?;
    Ok(())
}

pub trait Template {
    fn name(&self) -> &'static str;
    fn themes(&self) -> Vec<&'static str>;
    fn generate_config(&self, dst: &PathBuf, db: &Dao) -> Result<()>;
    fn generate_page(&self, dst: &PathBuf, page: &Page) -> Result<()>;
    fn build_script(&self, src: &PathBuf, dst: &PathBuf) -> Result<String>;
}

pub trait Query {
    fn ids(&self, db: &Connection) -> Result<Vec<i32>>;
    fn page(&self, db: &Connection, id: &i32) -> Result<Page>;
}

pub struct Page {
    pub name: String,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub published_at: NaiveDateTime,
}
