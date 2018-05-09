use rocket_contrib::Template;

use super::super::super::i18n::Locale;
use super::super::super::result::Result;
use super::application_layout;

#[get("/")]
fn index(lng: Locale) -> Result<Template> {
    let lyt = application_layout(&lng.name)?;
    Ok(Template::render("index", lyt))
}
