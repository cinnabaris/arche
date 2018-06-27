use std::ops::Deref;

use chrono::NaiveDateTime;
use diesel::prelude::*;

use super::super::{context::Context, errors::Result, orm::schema::schema_migrations, rfc::RFC822};

pub fn versions() -> Result<()> {
    let ctx = Context::new(&super::parse_config()?)?;
    let db = ctx.db.get()?;
    println!("{:16} {}", "VERSION", "RUN ON");
    for (v, r) in schema_migrations::dsl::schema_migrations
        .select((
            schema_migrations::dsl::version,
            schema_migrations::dsl::created_at,
        ))
        .load::<(String, NaiveDateTime)>(db.deref())?
    {
        println!("{:16} {}", v, r.to_rfc822());
    }

    Ok(())
}
