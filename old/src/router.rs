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

use super::{
    dao::{Connection, Dao},
    env, graphql, i18n,
    jwt::Home,
    plugins,
    result::Result,
};

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    let mut items = Vec::new();
    items.push(("/", routes!(sitemap, robots, rss)));
    // items.extend_from_slice(plugins::nut::routes().as_slice());
    // items.extend_from_slice(plugins::blog::routes().as_slice());
    // items.extend_from_slice(plugins::cbeta::routes().as_slice());
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

#[get("/rss/<lang>")]
fn rss(db: Connection, home: Home, lang: &RawStr) -> Result<Xml<Vec<u8>>> {
    let Home(home) = home;
    let lang = s!(lang);
    let db = db.deref();
    let db = Dao::new(db);

    let mut items = Vec::new();
    items.extend_from_slice(plugins::nut::rss(&lang).as_slice());

    let mut fields = Vec::new();
    for (url, title, desc, last) in items {
        fields.push(
            ItemBuilder::default()
                .link(format!("{}{}", home, url))
                .title(title)
                .description(desc)
                .pub_date(DateTime::<Utc>::from_utc(last, Utc).to_rfc3339())
                .build()?,
        );
    }

    let mut buf = Vec::new();
    let ch = ChannelBuilder::default()
        .link(home.clone())
        .language(lang.clone())
        .pub_date(Utc::now().to_rfc3339())
        .title(i18n::t(&db, &lang, &s!("site.title"), None::<Value>))
        .description(i18n::t(&db, &lang, &s!("site.description"), None::<Value>))
        .copyright(i18n::t(&db, &lang, &s!("site.copyright"), None::<Value>))
        .items(fields)
        .build()?;

    ch.write_to(&mut buf)?;

    Ok(Xml(buf))
}
