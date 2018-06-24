// https://gohugo.io/documentation/

use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use handlebars::Handlebars;
use log;
use serde_json;
use toml;

use super::super::{
    dao::Dao, result::{Error, Result}, rfc::RFC3399,
};
use super::Page;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
}

pub struct Template {}

impl super::Template for Template {
    fn name(&self) -> &'static str {
        "hugo"
    }
    fn themes(&self) -> Vec<&'static str> {
        vec![
            "bootstrap",
            "bulma",
            "materialize",
            "tachyons",
            "semantic-ui",
        ]
    }

    fn generate_config(&self, dst: &PathBuf, _db: &Dao) -> Result<()> {
        let cfg = Config { name: s!("") };
        let buf = toml::to_vec(&cfg)?;

        let file = dst.join("config.toml");
        log::info!("generate file {}", file.display());
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(file)?;
        file.write_all(&buf)?;
        Ok(())
    }

    fn generate_page(&self, dst: &PathBuf, page: &Page) -> Result<()> {
        let mut fd = OpenOptions::new()
            .read(true)
            .open(Path::new("templates").join("hugo.md.hbs"))?;
        let mut buf = String::new();
        fd.read_to_string(&mut buf)?;

        let body = Handlebars::new().render_template(
            &buf,
            &json!({
                    "title": page.title,
                    "body": page.body,
                    "published_at": page.published_at.to_rfc3399(),
                    "tags": serde_json::to_string(&page.tags)?,
                    "categories": serde_json::to_string(&page.categories)?,
                }),
        )?;

        let file = dst
            .join("content")
            .join("post")
            .join(&page.name)
            .with_extension("md");
        log::info!("generate file {}", file.display());

        let mut tpl = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o644)
            .open(file)?;
        tpl.write_all(body.as_bytes())?;
        Ok(())
    }

    fn build_script(&self, src: &PathBuf, dst: &PathBuf) -> Result<String> {
        if let Some(src) = src.to_str() {
            if let Some(dst) = dst.to_str() {
                return Ok(format!("hugo -s {src} -d {dst}", src = src, dst = dst));
            }
        }
        Err(Error::WithDescription(s!("bad filesystem path")))
    }
}
