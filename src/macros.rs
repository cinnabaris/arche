#[macro_export]
macro_rules! l {
    ($d:expr, $u:expr, $i:expr, $l:expr, $c:expr) => {
        l!($d, $u, $i, $l, $c, &None::<String>)
    };
    ($d:expr, $u:expr, $i:expr, $l:expr, $c:expr, $a:expr) => {
        ::plugins::nut::dao::log::add($d, $u, $i, &t!($d, $l, &$c.to_string(), $a))
    };
}

#[macro_export]
macro_rules! t {
    ($d:expr, $l:expr, $c:expr) => {
        t!($d, $l, $c, &None::<String>)
    };
    ($d:expr, $l:expr, $c:expr, $a:expr) => {
        ::i18n::t($d, $l, &$c.to_string(), $a)
    };
}

#[macro_export]
macro_rules! e {
    ($d:expr, $l:expr, $c:expr) => {
        e!($d, $l, $c, &None::<String>)
    };
    ($d:expr, $l:expr, $c:expr, $a:expr) => {
        t!($d, $l, &$c.to_string(), $a).into()
    };
}
