#[macro_export]
macro_rules! s {
    ($x:expr) => {
        $x.to_string()
    };
}

// #[macro_export]
// macro_rules! t {
//     ($db:expr, $lang:expr, $code:expr, $args:expr) => {
//         if let Ok(msg) = Locale::get_message($db, $lang, $code) {
//             if let Some(args) = $args {
//                 if let Ok(msg) = Handlebars::new().render_template(&msg, &args) {
//                     return msg;
//                 }
//             } else {
//                 return msg;
//             }
//         }
//         return format!("{}.{}", $lang, $code);
//     };
// }

// #[macro_export]
// macro_rules! e {
//     ($kind: expr, $db: expr, $lang: expr, $code: expr, $args: expr) => {
//         Err(Error::from(($kind, t!($db, $lang, $code, $args))));
//     };
// }
//
//
// #[macro_export]
// macro_rules! l {
//     ($t:expr, $u:expr, $l:expr, $i:expr, $c:expr, $a:expr) => {
//         try!(Log::add($t, $u, $l, $i, $c, $a));
//     };
// }
