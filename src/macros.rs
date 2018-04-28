#[macro_export]
macro_rules! s {
    ($x:expr) => {
        $x.to_string()
    };
}

#[macro_export]
macro_rules! gq {
    ($x:expr, $y:expr) => {
        ge!($y.call(&$x.context()))
    };
}

#[macro_export]
macro_rules! ge {
    ($x:expr) => {
        match $x {
            Ok(v) => Ok(v),
            Err(e) => Err(juniper::FieldError::new(e, juniper::Value::Null)),
        }
    };
}
