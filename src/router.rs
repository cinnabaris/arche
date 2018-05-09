use std::ops::Deref;
use std::path::{Path, PathBuf};

use chrono::{DateTime, FixedOffset, Utc};
use robots_txt::Robots;
use rocket::http::{RawStr, Status};
use rocket::response::{content::Xml, NamedFile};
use rocket::{self, Catcher, Route, State};
use rss::{ChannelBuilder, ItemBuilder};
use serde_json::Value;
use sitemap::structs::UrlEntry;
use sitemap::writer::SiteMapWriter;

use super::{env, graphql, i18n, jwt::Home, orm::PooledConnection as Db, plugins, result::Result};

pub fn catchers() -> Vec<Catcher> {
    errors![not_found, bad_request, forbidden, internal_server]
}

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    let mut items = Vec::new();
    items.push(("/", routes!(sitemap, robots, rss)));
    items.extend_from_slice(plugins::nut::routes().as_slice());
    items.extend_from_slice(plugins::blog::routes().as_slice());
    items.extend_from_slice(graphql::routes().as_slice());

    items
}

//-----------------------------------------------------------------------------

// https://en.wikipedia.org/wiki/Site_map
// https://www.sitemaps.org/protocol.html
#[get("/sitemap.xml")]
fn sitemap<'a>(home: Home) -> Result<Xml<Vec<u8>>> {
    let mut items = Vec::new();
    items.extend_from_slice(plugins::nut::sitemap().as_slice());
    items.extend_from_slice(plugins::blog::sitemap()?.as_slice());

    let Home(home) = home;
    let mut buf = Vec::new();
    {
        let fix = FixedOffset::east(0);
        let smw = SiteMapWriter::new(&mut buf);
        let mut uw = smw.start_urlset()?;

        for (url, pri, freq, last) in items {
            uw.url(
                UrlEntry::builder()
                    .loc(format!("{}{}", home, url))
                    .lastmod(DateTime::<Utc>::from_utc(last, Utc).with_timezone(&fix))
                    .priority(pri)
                    .changefreq(freq),
            )?;
        }

        uw.end()?;
    }

    Ok(Xml(buf))
}

// https://en.wikipedia.org/wiki/Robots_exclusion_standard
#[get("/robots.txt")]
fn robots(home: Home) -> Result<String> {
    let Home(home) = home;
    Ok(format!(
        "{}",
        Robots::start_build()
            .host(home.clone())
            .start_section_for("*")
            .disallow("/my/")
            .sitemap((home + "/sitemap.xml").parse()?)
            .end_section()
            .finalize()
    ))
}

#[get("/rss/<lang>")]
fn rss(db: Db, home: Home, lang: &RawStr) -> Result<Xml<Vec<u8>>> {
    let Home(home) = home;
    let lang = s!(lang);
    let db = db.deref();

    let mut items = Vec::new();
    items.extend_from_slice(plugins::blog::rss(&lang).as_slice());

    let mut fields = Vec::new();
    for (url, title, desc, last) in items {
        fields.push(ItemBuilder::default()
            .link(format!("{}{}", home, url))
            .title(title)
            .description(desc)
            .pub_date(DateTime::<Utc>::from_utc(last, Utc).to_rfc3339())
            .build()?);
    }

    let mut buf = Vec::new();
    let ch = ChannelBuilder::default()
        .link(home.clone())
        .language(lang.clone())
        .pub_date(Utc::now().to_rfc3339())
        .title(i18n::t(db, &lang, &s!("site.title"), None::<Value>))
        .description(i18n::t(db, &lang, &s!("site.description"), None::<Value>))
        .copyright(i18n::t(db, &lang, &s!("site.copyright"), None::<Value>))
        .items(fields)
        .build()?;

    ch.write_to(&mut buf)?;

    Ok(Xml(buf))
}

#[get("/assets/<file..>")]
pub fn get_assets(cfg: State<env::Config>, file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(
        Path::new("themes")
            .join(&cfg.http.theme)
            .join("assets")
            .join(file),
    ).ok()
}

#[error(404)]
fn not_found() -> &'static str {
    Status::NotFound.reason
}

#[error(400)]
fn bad_request() -> &'static str {
    Status::BadRequest.reason
}

#[error(403)]
fn forbidden() -> &'static str {
    Status::Forbidden.reason
}

#[error(500)]
fn internal_server() -> &'static str {
    Status::InternalServerError.reason
}
