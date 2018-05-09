use std::ffi::OsStr;
use std::fs::{read_dir, DirEntry, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::UNIX_EPOCH;

use chrono::{NaiveDateTime, TimeZone, Utc};

use super::super::super::result::{Error, Result};

fn walk_md(dir: &Path, cb: &Fn(&DirEntry) -> Result<()>) -> Result<()> {
    if dir.is_dir() {
        for it in read_dir(dir)? {
            let it = it?;
            let pt = it.path();
            if pt.is_dir() {
                walk_md(&pt, cb)?;
            } else {
                cb(&it)?;
            }
        }
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Markdown {
    pub title: String,
    pub uri: String,
    pub updated_at: NaiveDateTime,
}

impl Markdown {
    pub const EXT: &'static str = "md";

    pub fn new(root: &Path, file: &DirEntry) -> Result<Self> {
        let mt = file.metadata()?;
        let dur = mt.modified()?.duration_since(UNIX_EPOCH)?;
        let pt = file.path();
        if pt.extension() == Some(OsStr::new(Self::EXT)) {
            if let Some(name) = pt.strip_prefix(root)?.to_str() {
                let mut title = String::new();
                let mut fd = File::open(file.path())?;
                let mut fd = BufReader::new(fd);
                fd.read_line(&mut title)?;
                return Ok(Self {
                    title: s!(title.trim_left_matches('#').trim()),
                    uri: s!(name),
                    updated_at: Utc.timestamp(dur.as_secs() as i64, dur.subsec_nanos())
                        .naive_utc(),
                });
            }
        }
        Err(Error::WithDescription(format!("bad file {:?}", pt)))
    }

    pub fn read<P: AsRef<Path>>(uri: P) -> Result<(String, String)> {
        let mut title = String::new();
        let fd = File::open(path(Some(uri)))?;
        let mut buf = BufReader::new(fd);
        buf.read_line(&mut title)?;
        let title = s!(title.trim_left_matches('#').trim());

        let mut body = String::new();
        buf.seek(SeekFrom::Start(0))?;
        buf.read_to_string(&mut body)?;
        Ok((title, body))
    }
}

pub fn files() -> Result<Vec<Markdown>> {
    let items = Arc::new(Mutex::new(Vec::new()));

    let root = path(None::<String>);
    walk_md(root.as_path(), &|file| {
        let items = items.clone();
        let items = items.lock();
        match items {
            Ok(mut items) => {
                if let Ok(it) = Markdown::new(root.as_path(), file) {
                    items.push(it);
                }
                Ok(())
            }
            Err(e) => Err(Error::WithDescription(format!("{:?}", e))),
        }
    })?;

    let items = items.clone();
    let items = items.lock();
    match items {
        Ok(items) => Ok(items.deref().to_vec()),
        Err(e) => Err(Error::WithDescription(format!("{:?}", e))),
    }
}

pub fn path<P: AsRef<Path>>(n: Option<P>) -> PathBuf {
    let p = Path::new("tmp").join("blog");
    match n {
        Some(n) => p.join(n),
        None => p,
    }
}
