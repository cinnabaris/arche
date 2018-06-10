use std::{fs::OpenOptions, io::BufReader, path::PathBuf};

use csv;

use super::super::super::result::{Error, Result};
use super::{read_utf16_file, trim};

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Bookcase {
    series: Vec<Series>,
}

// https://www.cbeta.org/format/id.php
#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    pub code: String,
    pub short_name: String,
    pub chinese_name: String,
    pub english_name: String,
}

impl Series {
    pub fn load(root: &PathBuf) -> Result<Vec<Self>> {
        let buf = read_utf16_file(&root.join("bookdata.txt"))?;
        let rdr = BufReader::new(buf.as_bytes());

        let mut items = Vec::new();

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(rdr);
        for it in rdr.records() {
            let it = it?;
            if it.len() != 5 {
                return Err(Error::WithDescription(format!("Bad series line: {:?}", it)));
            }
            items.push(Self {
                code: trim(&it[0]),
                short_name: trim(&it[2]),
                chinese_name: trim(&it[3]),
                english_name: trim(&it[4]),
            });
        }

        Ok(items)
    }
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Spine {
    pub code: String,
    pub href: String,
}

impl Spine {
    pub fn load(root: &PathBuf) -> Result<Vec<Self>> {
        let fd = OpenOptions::new().read(true).open(&root.join("spine.txt"))?;

        let mut items = Vec::new();

        let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(fd);
        for it in rdr.records() {
            let it = it?;
            if it.len() != 2 {
                return Err(Error::WithDescription(format!("Bad spine line: {:?}", it)));
            }
            items.push(Self {
                code: trim(&it[1]),
                href: trim(&it[0]),
            });
        }

        Ok(items)
    }
}

// https://www.cbeta.org/format/linehead.php

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Catalog {
    pub series_code: String,
    pub class: String,
    pub id: String,
    pub code: String,
    pub len: String,
    pub title: String,
    pub author: String,
}

impl Catalog {
    pub fn load(root: &PathBuf) -> Result<Vec<Self>> {
        let fd = OpenOptions::new()
            .read(true)
            .open(&root.join("catalog.txt"))?;

        let mut items = Vec::new();

        let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(fd);
        for it in rdr.records() {
            let it = it?;
            if it.len() != 8 {
                return Err(Error::WithDescription(format!(
                    "Bad catalog line: {:?}",
                    it
                )));
            }
            items.push(Self {
                series_code: trim(&it[0]),
                class: trim(&it[1]),
                id: trim(&it[3]),
                code: trim(&it[4]),
                len: trim(&it[5]),
                title: trim(&it[6]),
                author: trim(&it[7]),
            });
        }

        Ok(items)
    }
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BookNav {}

impl BookNav {
    pub fn load(root: &PathBuf) -> Result<Vec<Self>> {
        let fd = OpenOptions::new()
            .read(true)
            .open(&root.join("book_nav.xhtml"))?;

        let mut items = Vec::new();
        // TODO
        Ok(items)
    }
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CatalogNav {}

impl CatalogNav {
    pub fn load(root: &PathBuf) -> Result<Vec<Self>> {
        let fd = OpenOptions::new()
            .read(true)
            .open(&root.join("bulei_nav.xhtml"))?;

        let mut items = Vec::new();
        // TODO
        Ok(items)
    }
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Book {}

impl Book {
    pub fn load(root: &PathBuf, href: &String) -> Result<Vec<Self>> {
        let fd = OpenOptions::new()
            .read(true)
            .open(&root.join("XML").join(href))?;

        let mut items = Vec::new();
        // TODO
        Ok(items)
    }
}
