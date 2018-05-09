use chrono::{NaiveDateTime, Utc};

pub fn rss(_lang: &String) -> Vec<(String, String, String, NaiveDateTime)> {
    vec![(
        s!("/blog/readme.md"),
        s!("README"),
        s!("CONTENT"),
        Utc::now().naive_utc(),
    )]
}
