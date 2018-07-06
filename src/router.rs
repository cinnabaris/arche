use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;
use std::sync::Arc;

use chrono::{DateTime, FixedOffset, Utc};
use robots_txt::Robots;
use rocket::{
    http::Status,
    response::{content::Xml, NamedFile},
    Catcher, Route, State,
};
use rss::{ChannelBuilder, ItemBuilder};
use sitemap::{structs::UrlEntry, writer::SiteMapWriter, Error as SitemapError};

use super::{
    context::Context,
    errors::Result,
    graphql,
    orm::PooledConnection as Db,
    plugins::{forum, nut},
    request::Home,
};

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    let mut items = Vec::new();
    items.extend_from_slice(&nut::routes());
    items.extend_from_slice(&forum::routes());
    items.push(graphql::routes());
    items.push((
        "/",
        routes![
            third,
            upload,
            global,
            assets,
            robots_txt,
            sitemap_xml_gz,
            rss_atom
        ],
    ));
    items
}

pub fn catchers() -> Vec<Catcher> {
    errors![not_found, forbidden, internal_server_error]
}

#[get("/global/<file..>")]
fn global(file: PathBuf) -> Result<NamedFile> {
    Ok(NamedFile::open(
        Path::new("themes").join("global").join(file),
    )?)
}

#[get("/assets/<file..>")]
fn assets(file: PathBuf, ctx: State<Arc<Context>>) -> Result<NamedFile> {
    Ok(NamedFile::open(
        Path::new("themes")
            .join(ctx.config.http.theme.clone())
            .join("assets")
            .join(file),
    )?)
}

#[get("/3rd/<file..>")]
fn third(file: PathBuf) -> Result<NamedFile> {
    Ok(NamedFile::open(Path::new("node_modules").join(file))?)
}

#[get("/upload/<file..>")]
fn upload(file: PathBuf) -> Result<NamedFile> {
    Ok(NamedFile::open(Path::new("tmp").join("upload").join(file))?)
}

// https://en.wikipedia.org/wiki/Robots_exclusion_standard
#[get("/robots.txt")]
fn robots_txt(home: Home) -> Result<String> {
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

fn build_sitemap(home: &String, buf: &mut Vec<u8>) -> StdResult<(), SitemapError> {
    let srt = SiteMapWriter::new(buf);
    let mut urt = srt.start_urlset()?;
    let fix = FixedOffset::east(0);

    for items in vec![nut::sitemap(), forum::sitemap()] {
        for (loc, priority, changefreq, lastmod) in items {
            urt.url(
                UrlEntry::builder()
                    .loc(format!("{}{}", home, loc))
                    .priority(priority)
                    .changefreq(changefreq)
                    .lastmod(DateTime::<Utc>::from_utc(lastmod, Utc).with_timezone(&fix)),
            )?;
        }
    }

    urt.end()?;
    Ok(())
}

// https://en.wikipedia.org/wiki/Site_map
// https://www.sitemaps.org/protocol.html
#[get("/sitemap.xml")]
fn sitemap_xml_gz(home: Home) -> Result<Xml<Vec<u8>>> {
    let Home(home) = home;
    let mut buf = Vec::new();
    match build_sitemap(&home, &mut buf) {
        Ok(_) => Ok(Xml(buf)),
        Err(e) => Err(format!("{:?}", e).into()),
    }
}
// fn sitemap_xml_gz<'r>() -> response::Result<'r> {
//     let mut buf = Cursor::new(Vec::new());
//     match build_sitemap(&mut buf) {
//         Ok(_) => Response::build()
//             .sized_body(buf)
//             .header(ContentType::XML)
//             .header(ContentType::new("application", "x-gzip"))
//             .header(Header::from(ContentEncoding(vec![Encoding::Gzip])))
//             .header(Header::from(ContentDisposition {
//                 disposition: DispositionType::Attachment,
//                 parameters: vec![DispositionParam::Filename(
//                     Charset::Us_Ascii,
//                     None,
//                     b"sitemap.xml.gz".to_vec(),
//                 )],
//             }))
//             .ok(),
//         Err(e) => {
//             log::error!("{:?}", e);
//             Err(Status::InternalServerError)
//         }
//     }
// }

#[get("/rss/<lang>")]
fn rss_atom(db: Db, home: Home, lang: String) -> Result<Xml<Vec<u8>>> {
    let Home(home) = home;
    let db = db.deref();
    let mut fields = Vec::new();
    for items in vec![nut::rss(&lang), forum::rss(&lang)] {
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
    }
    let mut buf = Vec::new();
    let ch = ChannelBuilder::default()
        .link(home.clone())
        .language(lang.clone())
        .pub_date(Utc::now().to_rfc3339())
        .title(t!(db, &lang, "site.title"))
        .description(t!(db, &lang, "site.description"))
        .copyright(t!(db, &lang, "site.copyright"))
        .items(fields)
        .build()?;

    ch.write_to(&mut buf)?;

    Ok(Xml(buf))
}

#[error(404)]
fn not_found() -> &'static str {
    Status::NotFound.reason
}

#[error(403)]
fn forbidden() -> &'static str {
    Status::Forbidden.reason
}

#[error(500)]
fn internal_server_error() -> &'static str {
    Status::InternalServerError.reason
}
