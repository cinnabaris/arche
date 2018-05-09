use chrono::{DateTime, Local, NaiveDateTime};

pub trait RFC822 {
    fn to_rfc822(&self) -> String;
}

impl RFC822 for NaiveDateTime {
    fn to_rfc822(&self) -> String {
        // "%c"
        // "%a, %d %b %Y %T GMT"
        // "%a, %d %b %Y %T %Z"
        DateTime::<Local>::from_utc(*self, *Local::now().offset())
            .format("%a, %d %b %Y %T %Z")
            .to_string()
    }
}
