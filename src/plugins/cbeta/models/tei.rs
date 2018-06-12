// https://www.cbeta.org/format/xml.php
// https://www.tei-c.org/Guidelines/P5/

use std::{fs::OpenOptions, path::PathBuf};

use serde_xml_rs;

use super::super::super::super::result::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tei {
    pub id: String,
}

impl Tei {
    pub fn load(file: &PathBuf) -> Result<Self> {
        let fd = OpenOptions::new().read(true).open(file)?;
        let it: Tei = serde_xml_rs::deserialize(fd)?;
        Ok(it)
    }
}
