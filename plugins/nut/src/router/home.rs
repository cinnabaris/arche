use std::path::{Path, PathBuf};

use robots_txt::Robots;
use rocket::{response::NamedFile, State};

use super::super::{env, errors::Result, jwt::Home};

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

#[get("/assets/<file..>")]
pub fn assets(cfg: State<env::Config>, file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(
        Path::new("themes")
            .join(&cfg.http.theme)
            .join("assets")
            .join(file),
    ).ok()
}

#[get("/global/<file..>")]
pub fn global(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("themes").join("global").join(file)).ok()
}

#[get("/3rd/<file..>")]
pub fn third(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("node_modules").join(file)).ok()
}
