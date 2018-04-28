#[macro_export]
macro_rules! s {
    ($x:expr) => {
        $x.to_string()
    };
}

#[macro_export]
macro_rules! gq {
    ($x:expr, $y:expr) => {
        match $y.call(&$x.context()) {
            Ok(v) => Ok(v),
            Err(e) => Err(juniper::FieldError::new(e, juniper::Value::Null)),
        }
    };
}
