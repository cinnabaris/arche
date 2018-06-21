pub mod consumers;

use chrono::{NaiveDateTime, Utc};
use sitemap::structs::ChangeFreq;

pub fn sitemap() -> Vec<(String, f32, ChangeFreq, NaiveDateTime)> {
    vec![(s!("/"), 0.1, ChangeFreq::Daily, Utc::now().naive_utc())]
}

pub fn rss(_lang: &String) -> Vec<(String, String, String, NaiveDateTime)> {
    vec![]
}
