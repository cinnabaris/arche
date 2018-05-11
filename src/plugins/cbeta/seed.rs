use std::ffi::OsStr;
use std::fs::{read_dir, DirEntry};
use std::path::{Path, PathBuf};

use chrono::NaiveDate;
use diesel::Connection;
use epub;
use log;

use super::super::super::{
    orm::Connection as Db,
    result::{Error, Result},
};
use super::models::{Book, Page};

pub fn load(db: &Db) -> Result<()> {
    books(db, Path::new("tmp").join("books"))?;
    Ok(())
}

fn books(db: &Db, root: PathBuf) -> Result<()> {
    log::info!("load books from {:?}", root);
    walk(root.as_path(), &|file| {
        db.transaction::<_, Error, _>(|| parse(db, file.path()))
    })
}

fn parse(db: &Db, file: PathBuf) -> Result<()> {
    if file.extension() != Some(OsStr::new(EPUB)) {
        log::warn!("find book {:?}", file);
        return Ok(());
    }
    log::info!("find book {:?}", file);

    let bk = epub::open(file)?;
    let ct = bk.container()?;
    let opf = ct.opf();
    if opf.len() != 1 {
        return Err(Error::WithDescription(s!("bad opf content")));
    }
    let mt = bk.opf(&opf.first().unwrap())?.metadata;
    let published_at = NaiveDate::parse_from_str(&mt.date.content[..], "%Y-%m-%d")?;

    db.transaction::<_, Error, _>(|| {
        let bid = Book::set(
            db,
            &bk.uid()?,
            &mt.creator.content,
            &mt.publisher.content,
            &mt.title.content,
            &bk.mimetype()?,
            &mt.language.content,
            &published_at,
            &bk.index()?,
        )?;
        let n = Page::clear(db, &bid)?;
        log::info!("remove {} pages", n);
        for (h, t) in bk.list()? {
            let (b, _) = bk.show(&h)?;
            Page::add(db, &bid, &h, &b, &t)?;
        }
        Ok(())
    })
}

fn walk(dir: &Path, cb: &Fn(&DirEntry) -> Result<()>) -> Result<()> {
    if dir.is_dir() {
        for it in read_dir(dir)? {
            let it = it?;
            let pt = it.path();
            if pt.is_dir() {
                walk(&pt, cb)?;
            } else {
                cb(&it)?;
            }
        }
    }
    Ok(())
}

pub const EPUB: &'static str = "epub";
