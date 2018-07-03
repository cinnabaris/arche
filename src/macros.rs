#[macro_export]
macro_rules! l {
    ($d:expr, $u:expr, $i:expr, $l:expr, $c:expr) => {
        l!($d, $u, $i, $l, $c, &None::<String>)
    };
    ($d:expr, $u:expr, $i:expr, $l:expr, $c:expr, $a:expr) => {
        ::plugins::nut::dao::add_log($d, $u, $i, &::i18n::t($d, $l, &$c.to_string(), $a))
    };
}
