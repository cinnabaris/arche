use std::fs::File;
use std::io::prelude::*;
use std::process;

use super::errors::Result;

#[cfg(unix)]
pub fn hostname() -> Result<String> {
    read("/proc/sys/kernel/hostname")
}

#[cfg(unix)]
pub fn version() -> Result<String> {
    read("/proc/version")
}

pub fn read(file: &'static str) -> Result<String> {
    let mut fd = File::open(file)?;
    let mut buf = String::new();
    fd.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn pid() -> u32 {
    process::id()
}
