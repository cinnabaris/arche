#![feature(
    plugin, use_extern_macros, custom_derive, custom_attribute, proc_macro_path_invoc,
    extern_prelude
)]
#![recursion_limit = "128"] // https://github.com/diesel-rs/diesel/issues/1127

#[macro_use]
pub mod macros;

pub mod app;
pub mod cache;
pub mod context;
pub mod dao;
pub mod env;
pub mod graphql;
pub mod i18n;
pub mod jwt;
pub mod pagination;
pub mod plugins;
pub mod queue;
pub mod result;
pub mod rfc;
pub mod router;
pub mod security;
pub mod settings;
pub mod themes;
