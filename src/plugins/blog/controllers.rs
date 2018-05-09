use rocket_contrib::Template;

use super::super::super::{i18n::Locale, result::Result};
use super::super::nut::application_layout;
use super::seed;

#[get("/")]
fn index(lng: Locale) -> Result<Template> {
    let mut lyt = application_layout(&lng.name)?;
    lyt.insert(s!("items"), json!(seed::files()?));
    Ok(Template::render("blog/index", lyt))
}
