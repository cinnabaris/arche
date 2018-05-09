use robots_txt::Robots;
use rocket::http::RawStr;
use rocket_contrib::Template;

use super::super::super::i18n::Locale;
use super::super::super::jwt::Home;
use super::super::super::result::Result;
use super::application_layout;

#[get("/")]
fn index(lng: Locale) -> Result<Template> {
    let lyt = application_layout(&lng.name)?;
    Ok(Template::render("index", lyt))
}

// https://en.wikipedia.org/wiki/Site_map
// https://www.sitemaps.org/protocol.html
#[get("/sitemap.xml.gz")]
fn sitemap(lng: Locale) -> Result<Template> {
    let lyt = application_layout(&lng.name)?;
    Ok(Template::render("index", lyt))
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
            .sitemap((home + "/sitemap.xml.gz").parse()?)
            .end_section()
            .finalize()
    ))
}

#[get("/rss/<lang>")]
fn rss(lang: &RawStr) -> Result<Template> {
    let lyt = application_layout(&s!(lang))?;
    Ok(Template::render("index", lyt))
}
