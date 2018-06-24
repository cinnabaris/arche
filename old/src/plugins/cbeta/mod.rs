// https://www.cbeta.org/format/index.php

pub mod models;

use std::{
    fs::OpenOptions, io::{Cursor, Read}, path::PathBuf,
};

use byteorder::{LittleEndian, ReadBytesExt};

use super::super::result::Result;

pub fn read_utf16_file(file: &PathBuf) -> Result<String> {
    let mut fd = OpenOptions::new().read(true).open(file)?;
    let mut u8f = Vec::new();
    fd.read_to_end(&mut u8f)?;
    let mut u16f = vec![0; u8f.len() / 2];
    let mut rdr = Cursor::new(u8f);
    rdr.read_u16_into::<LittleEndian>(&mut u16f)?;
    Ok(String::from_utf16(u16f.as_slice())?)
}

pub fn trim(s: &str) -> String {
    s!(s!(s).trim())
}
