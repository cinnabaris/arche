extern crate arche;

use std::fmt::Debug;
use std::path::{Path, PathBuf};

#[test]
fn nav() {
    let root = root();
    for it in vec!["book_nav.xhtml", "bulei_nav.xhtml", "toc/A/A1066.xml"] {
        let nav = arche::plugins::cbeta::models::nav::Html::load(&root.join(it)).unwrap();
        println!("=== nav {} ===", nav.head.title.text);
        // println!("{:?}", nav);
    }
}

#[test]
fn csv() {
    let root = root();

    print(
        "SERIES",
        arche::plugins::cbeta::models::Series::load(&root).unwrap(),
    );
    print(
        "SPINE",
        arche::plugins::cbeta::models::Spine::load(&root).unwrap(),
    );
    print(
        "CATALOG",
        arche::plugins::cbeta::models::Catalog::load(&root).unwrap(),
    );
}

fn print<T: Debug>(name: &'static str, items: Vec<T>) {
    println!("======= {} {} =======", name, items.len());
    println!("{:?}", items[0]);
}

fn root() -> PathBuf {
    Path::new("tmp")
        .join("cbeta")
        .join("Bookcase")
        .join("CBETA")
}
