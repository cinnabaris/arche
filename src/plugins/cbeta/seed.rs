use std::fs::{read_dir, DirEntry};
use std::path::{Path, PathBuf};

use diesel::Connection;
use epub;
use log;

use super::super::super::{
    orm::Connection as Db,
    result::{Error, Result},
};
use super::models::Book;

pub fn load(db: &Db) -> Result<()> {
    books(db, Path::new("tmp").join("books"))?;
    Ok(())
}

fn books(db: &Db, root: PathBuf) -> Result<()> {
    log::info!("load books from {:?}", root);
    walk(root.as_path(), &|file| {
        log::info!("find book {:?}", file);
        let it = epub::open(file.path())?;
        db.transaction::<_, Error, _>(|| {
            Book::add(db, &it)?;
            Ok(())
        })
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
