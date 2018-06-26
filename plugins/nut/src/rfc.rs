use chrono::{DateTime, Local, NaiveDateTime, Utc as UTC};

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

// ISO 8601 / RFC 3339
pub trait RFC3399 {
    fn to_rfc3399(&self) -> String;
}

impl RFC3399 for NaiveDateTime {
    fn to_rfc3399(&self) -> String {
        DateTime::<Local>::from_utc(*self, *Local::now().offset())
            .format("%+")
            .to_string()
    }
}

pub trait Utc {
    fn to_utc(&self) -> DateTime<UTC>;
}

impl Utc for NaiveDateTime {
    fn to_utc(&self) -> DateTime<UTC> {
        DateTime::<UTC>::from_utc(*self, *UTC::now().offset())
    }
}
