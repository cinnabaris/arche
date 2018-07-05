use log;

use super::super::{context::Context, errors::Result};

pub fn list() -> Result<()> {
    let ctx = Context::new(&super::parse_config()?)?;
    let items = ctx.cache.keys()?;
    println!("{:64} {}", "KEY", "TTL");
    for (key, ttl) in items {
        println!("{:64} {}", key, ttl);
    }
    Ok(())
}

pub fn clear() -> Result<()> {
    let ctx = Context::new(&super::parse_config()?)?;
    let cnt = ctx.cache.clear()?;
    log::info!("remove {} items from cache", cnt);
    Ok(())
}
