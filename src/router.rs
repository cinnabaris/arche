use rocket::Route;

use super::plugins::nut;

pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
    let mut items = Vec::new();
    items.push(nut::routes());
    items.push(("/", routes![robots_txt, sitemap_xml_gz, rss_atom]));
    items
}

#[get("/robots.txt")]
fn robots_txt() -> &'static str {
    "robots"
}

#[get("/sitemap.xml.gz")]
fn sitemap_xml_gz() -> &'static str {
    "sitemap"
}

#[get("/rss/<lang>")]
fn rss_atom(lang: String) -> &'static str {
    "rss"
}
