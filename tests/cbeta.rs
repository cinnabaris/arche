extern crate arche;

use std::fmt::Debug;
use std::path::{Path, PathBuf};

#[test]
fn nav() {
    let root = root();
    for it in vec!["book_nav.xhtml", "bulei_nav.xhtml", "toc/A/A1066.xml"] {
        let it = arche::plugins::cbeta::models::nav::Html::load(&root.join(it)).unwrap();
        println!("=== nav {} ===", it.head.title.text);
        // println!("{:?}", nav);
    }

    for it in vec!["XML/Y/Y01/Y01n0001_001.xml"] {
        let it = arche::plugins::cbeta::models::tei::Tei::load(&root.join(it)).unwrap();
        println!("=== tei {} ===", it.id);
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
