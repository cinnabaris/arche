use std::error::Error as StdError;
use std::{env, error, fmt, io, num, path, result, string, time};

#[cfg(feature = "mq-amqp")]
use amqp;
use base64;
use chrono;
use clap;
use csv;
use diesel;
use epub;
use frank_jwt;
use handlebars;
use ini;
use language_tags;
use lettre;
use lettre_email;
use log;
use log4rs;
use r2d2;
#[cfg(feature = "ch-redis")]
use redis;
use regex;
use rocket;
use rss;
use serde_json;
use serde_xml_rs;
use sitemap;
use stardict;
use sys_info;
use toml;
use url;
use validator;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    EnvVar(env::VarError),
    NumParseInt(num::ParseIntError),
    #[cfg(feature = "ch-redis")]
    Redis(redis::RedisError),
    R2d2(r2d2::Error),
    HandlebarsTemplateRender(handlebars::TemplateRenderError),
    HandlebarsTemplateFile(handlebars::TemplateFileError),
    HandlebarsRender(handlebars::RenderError),
    RocketConfig(rocket::config::ConfigError),
    RocketLaunch(rocket::error::LaunchError),
    Ini(ini::ini::Error),
    Epub(epub::result::Error),
    StarDict(stardict::result::Error),
    SystemTime(time::SystemTimeError),
    FrankJwt(frank_jwt::Error),
    SerdeJson(serde_json::Error),
    #[cfg(feature = "mq-rabbit")]
    Amqp(amqp::AMQPError),
    Base64Decode(base64::DecodeError),
    StringFromUtf8(string::FromUtf8Error),
    StringFromUtf16(string::FromUtf16Error),
    SerdeXml(serde_xml_rs::Error),
    Log4rs(log4rs::Error),
    UrlParse(url::ParseError),
    Regex(regex::Error),
    Clap(clap::Error),
    LettreSmtp(lettre::smtp::error::Error),
    LettreEmail(lettre_email::error::Error),
    LanguageTags(language_tags::Error),
    SysInfo(sys_info::Error),
    TomlDe(toml::de::Error),
    TomlSer(toml::ser::Error),
    Diesel(diesel::result::Error),
    Validation(validator::ValidationErrors),
    ChronoParse(chrono::ParseError),
    Rss(rss::Error),
    PathStripPrefixError(path::StripPrefixError),
    Csv(csv::Error),
    WithDescription(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::FrankJwt(ref err) => write!(f, "{:?}", err),
            Error::Io(ref err) => err.fmt(f),
            Error::NumParseInt(ref err) => err.fmt(f),
            #[cfg(feature = "ch-redis")]
            Error::Redis(ref err) => err.fmt(f),
            Error::R2d2(ref err) => err.fmt(f),
            Error::HandlebarsTemplateRender(ref err) => err.fmt(f),
            Error::HandlebarsTemplateFile(ref err) => err.fmt(f),
            Error::HandlebarsRender(ref err) => err.fmt(f),
            Error::RocketConfig(ref err) => err.fmt(f),
            Error::RocketLaunch(ref err) => err.fmt(f),
            Error::Ini(ref err) => err.fmt(f),
            Error::Epub(ref err) => err.fmt(f),
            Error::StarDict(ref err) => err.fmt(f),
            Error::SystemTime(ref err) => err.fmt(f),
            Error::SerdeJson(ref err) => err.fmt(f),
            #[cfg(feature = "mq-rabbit")]
            Error::Amqp(ref err) => err.fmt(f),
            Error::Base64Decode(ref err) => err.fmt(f),
            Error::StringFromUtf8(ref err) => err.fmt(f),
            Error::StringFromUtf16(ref err) => err.fmt(f),
            Error::SerdeXml(ref err) => err.fmt(f),
            Error::Log4rs(ref err) => err.fmt(f),
            Error::UrlParse(ref err) => err.fmt(f),
            Error::Regex(ref err) => err.fmt(f),
            Error::Clap(ref err) => err.fmt(f),
            Error::LettreSmtp(ref err) => err.fmt(f),
            Error::LettreEmail(ref err) => err.fmt(f),
            Error::LanguageTags(ref err) => err.fmt(f),
            Error::SysInfo(ref err) => err.fmt(f),
            Error::EnvVar(ref err) => err.fmt(f),
            Error::TomlDe(ref err) => err.fmt(f),
            Error::TomlSer(ref err) => err.fmt(f),
            Error::Diesel(ref err) => err.fmt(f),
            Error::Validation(ref err) => err.fmt(f),
            Error::ChronoParse(ref err) => err.fmt(f),
            Error::Rss(ref err) => err.fmt(f),
            Error::PathStripPrefixError(ref err) => err.fmt(f),
            Error::Csv(ref err) => err.fmt(f),
            Error::WithDescription(ref desc) => write!(f, "{}", desc),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::FrankJwt(ref _err) => "bad jwt token",
            Error::Io(ref err) => err.description(),
            Error::NumParseInt(ref err) => err.description(),
            #[cfg(feature = "ch-redis")]
            Error::Redis(ref err) => err.description(),
            Error::R2d2(ref err) => err.description(),
            Error::HandlebarsTemplateRender(ref err) => err.description(),
            Error::HandlebarsTemplateFile(ref err) => err.description(),
            Error::HandlebarsRender(ref err) => err.description(),
            Error::RocketConfig(ref err) => err.description(),
            Error::RocketLaunch(ref err) => err.description(),
            Error::Ini(ref err) => err.description(),
            Error::Epub(ref err) => err.description(),
            Error::StarDict(ref err) => err.description(),
            Error::SystemTime(ref err) => err.description(),
            Error::SerdeJson(ref err) => err.description(),
            #[cfg(feature = "mq-rabbit")]
            Error::Amqp(ref err) => err.description(),
            Error::Base64Decode(ref err) => err.description(),
            Error::StringFromUtf8(ref err) => err.description(),
            Error::StringFromUtf16(ref err) => err.description(),
            Error::SerdeXml(ref err) => err.description(),
            Error::Log4rs(ref err) => err.description(),
            Error::UrlParse(ref err) => err.description(),
            Error::Regex(ref err) => err.description(),
            Error::Clap(ref err) => err.description(),
            Error::LettreSmtp(ref err) => err.description(),
            Error::LettreEmail(ref err) => err.description(),
            Error::LanguageTags(ref err) => err.description(),
            Error::SysInfo(ref err) => err.description(),
            Error::EnvVar(ref err) => err.description(),
            Error::TomlDe(ref err) => err.description(),
            Error::TomlSer(ref err) => err.description(),
            Error::Diesel(ref err) => err.description(),
            Error::Validation(ref err) => err.description(),
            Error::ChronoParse(ref err) => err.description(),
            Error::Rss(ref err) => err.description(),
            Error::PathStripPrefixError(ref err) => err.description(),
            Error::Csv(ref err) => err.description(),
            Error::WithDescription(ref desc) => &desc,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            #[cfg(feature = "ch-redis")]
            Error::Redis(ref err) => Some(err),
            Error::R2d2(ref err) => Some(err),
            Error::NumParseInt(ref err) => Some(err),
            Error::HandlebarsTemplateRender(ref err) => Some(err),
            Error::HandlebarsTemplateFile(ref err) => Some(err),
            Error::HandlebarsRender(ref err) => Some(err),
            Error::RocketConfig(ref err) => Some(err),
            Error::RocketLaunch(ref err) => Some(err),
            Error::Ini(ref err) => Some(err),
            Error::Epub(ref err) => Some(err),
            Error::StarDict(ref err) => Some(err),
            Error::SystemTime(ref err) => Some(err),
            Error::SerdeJson(ref err) => Some(err),
            #[cfg(feature = "mq-rabbit")]
            Error::Amqp(ref err) => Some(err),
            Error::Base64Decode(ref err) => Some(err),
            Error::StringFromUtf8(ref err) => Some(err),
            Error::StringFromUtf16(ref err) => Some(err),
            Error::SerdeXml(ref err) => Some(err),
            Error::Log4rs(ref err) => Some(err),
            Error::UrlParse(ref err) => Some(err),
            Error::Regex(ref err) => Some(err),
            Error::Clap(ref err) => Some(err),
            Error::LettreSmtp(ref err) => Some(err),
            Error::LettreEmail(ref err) => Some(err),
            Error::LanguageTags(ref err) => Some(err),
            Error::SysInfo(ref err) => Some(err),
            Error::EnvVar(ref err) => Some(err),
            Error::TomlDe(ref err) => Some(err),
            Error::TomlSer(ref err) => Some(err),
            Error::Diesel(ref err) => Some(err),
            Error::Validation(ref err) => Some(err),
            Error::ChronoParse(ref err) => Some(err),
            Error::PathStripPrefixError(ref err) => Some(err),
            Error::Csv(ref err) => Some(err),
            Error::Rss(ref err) => Some(err),
            Error::FrankJwt(ref _err) => None,
            Error::WithDescription(ref _err) => None,
        }
    }
}

impl<'r> rocket::response::Responder<'r> for Error {
    fn respond_to(
        self,
        req: &rocket::Request,
    ) -> result::Result<rocket::response::Response<'r>, rocket::http::Status> {
        log::error!("{:?}", self);
        rocket::Response::build()
            .header(rocket::http::ContentType::Plain)
            .status(rocket::http::Status::InternalServerError)
            .sized_body(io::Cursor::new(self.description().to_string()))
            .finalize()
            .respond_to(req)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

#[cfg(feature = "ch-redis")]
impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Error {
        Error::Redis(err)
    }
}

impl From<r2d2::Error> for Error {
    fn from(err: r2d2::Error) -> Error {
        Error::R2d2(err)
    }
}

impl From<handlebars::TemplateRenderError> for Error {
    fn from(err: handlebars::TemplateRenderError) -> Error {
        Error::HandlebarsTemplateRender(err)
    }
}

impl From<handlebars::TemplateFileError> for Error {
    fn from(err: handlebars::TemplateFileError) -> Error {
        Error::HandlebarsTemplateFile(err)
    }
}

impl From<handlebars::RenderError> for Error {
    fn from(err: handlebars::RenderError) -> Error {
        Error::HandlebarsRender(err)
    }
}

impl From<rocket::config::ConfigError> for Error {
    fn from(err: rocket::config::ConfigError) -> Error {
        Error::RocketConfig(err)
    }
}

impl From<rocket::error::LaunchError> for Error {
    fn from(err: rocket::error::LaunchError) -> Error {
        Error::RocketLaunch(err)
    }
}

impl From<ini::ini::Error> for Error {
    fn from(err: ini::ini::Error) -> Error {
        Error::Ini(err)
    }
}

impl From<stardict::result::Error> for Error {
    fn from(err: stardict::result::Error) -> Error {
        Error::StarDict(err)
    }
}

impl From<epub::result::Error> for Error {
    fn from(err: epub::result::Error) -> Error {
        Error::Epub(err)
    }
}

impl From<time::SystemTimeError> for Error {
    fn from(err: time::SystemTimeError) -> Error {
        Error::SystemTime(err)
    }
}

impl From<frank_jwt::Error> for Error {
    fn from(err: frank_jwt::Error) -> Error {
        Error::FrankJwt(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::SerdeJson(err)
    }
}
#[cfg(feature = "mq-rabbit")]
impl From<amqp::AMQPError> for Error {
    fn from(err: amqp::AMQPError) -> Error {
        Error::Amqp(err)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Error {
        Error::Base64Decode(err)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error {
        Error::StringFromUtf8(err)
    }
}

impl From<string::FromUtf16Error> for Error {
    fn from(err: string::FromUtf16Error) -> Error {
        Error::StringFromUtf16(err)
    }
}

impl From<serde_xml_rs::Error> for Error {
    fn from(err: serde_xml_rs::Error) -> Error {
        Error::SerdeXml(err)
    }
}

impl From<log4rs::Error> for Error {
    fn from(err: log4rs::Error) -> Error {
        Error::Log4rs(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::UrlParse(err)
    }
}

impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Error {
        Error::Regex(err)
    }
}

impl From<clap::Error> for Error {
    fn from(err: clap::Error) -> Error {
        Error::Clap(err)
    }
}

impl From<lettre::smtp::error::Error> for Error {
    fn from(err: lettre::smtp::error::Error) -> Error {
        Error::LettreSmtp(err)
    }
}

impl From<lettre_email::error::Error> for Error {
    fn from(err: lettre_email::error::Error) -> Error {
        Error::LettreEmail(err)
    }
}

impl From<language_tags::Error> for Error {
    fn from(err: language_tags::Error) -> Error {
        Error::LanguageTags(err)
    }
}

impl From<sys_info::Error> for Error {
    fn from(err: sys_info::Error) -> Error {
        Error::SysInfo(err)
    }
}

impl From<env::VarError> for Error {
    fn from(err: env::VarError) -> Error {
        Error::EnvVar(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::NumParseInt(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Error {
        Error::TomlDe(err)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Error {
        Error::TomlSer(err)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Error {
        Error::Diesel(err)
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(err: validator::ValidationErrors) -> Error {
        Error::Validation(err)
    }
}

impl From<chrono::ParseError> for Error {
    fn from(err: chrono::ParseError) -> Error {
        Error::ChronoParse(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::WithDescription(err)
    }
}

impl From<sitemap::Error> for Error {
    fn from(err: sitemap::Error) -> Error {
        Error::WithDescription(format!("{:?}", err))
    }
}

impl From<rss::Error> for Error {
    fn from(err: rss::Error) -> Error {
        Error::Rss(err)
    }
}

impl From<path::StripPrefixError> for Error {
    fn from(err: path::StripPrefixError) -> Error {
        Error::PathStripPrefixError(err)
    }
}

impl From<csv::Error> for Error {
    fn from(err: csv::Error) -> Error {
        Error::Csv(err)
    }
}
