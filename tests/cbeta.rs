extern crate arche;

use std::fmt::Debug;
use std::path::Path;

#[test]
fn load() {
    let root = Path::new("tmp")
        .join("cbeta")
        .join("Bookcase")
        .join("CBETA");

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
