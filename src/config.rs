use std::env;
use std::fmt::Formatter;
use std::ops::Add;
use std::path::PathBuf;
use std::str::FromStr;

use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use tokio::fs;
use tokio::io;
use tracing::Level;

#[derive(Deserialize)]
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct Mpd {
    pub host: String,
    pub port: u32,
    pub password: Option<String>,
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct Server {
    pub bind: String,
    pub port: u32,
}

fn deserialize_level<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Level, D::Error> {
    struct LevelVisitor;

    impl<'de> Visitor<'de> for LevelVisitor {
        type Value = Level;

        fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
            f.write_str("enum Level")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
            Level::from_str(v)
                .map_err(|e| serde::de::Error::custom(e.to_string()))
        }
    }

    deserializer.deserialize_str(LevelVisitor)
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct Logging {
    #[serde(deserialize_with = "deserialize_level")]
    pub level: Level,
}

#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct Config {
    pub mpd: Mpd,
    pub server: Server,
    pub logging: Logging,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            mpd: Mpd {
                host: "localhost".to_owned(),
                port: 6600,
                password: None,
            },
            server: Server {
                bind: "0.0.0.0".to_owned(),
                port: 8989,
            },
            logging: Logging {
                level: Level::INFO,
            },
        }
    }
}

fn path() -> Result<PathBuf, String> {
    let result = env::var("XDG_CONFIG_HOME")
        .or_else(|_| {
            env::var("HOME")
                .map(|home| home.add("/.config"))
        })
        .map(|dir| dir.add("/mpdweb/config.toml"))
        .map_err(|e| format!("failed to get config path: {e}"))?;

    Ok(PathBuf::from(result))
}

pub async fn read() -> Result<Config, String> {
    let path = path()?;

    match fs::read_to_string(&path).await {
        Ok(contents) => {
            toml::from_str(&contents)
                .map_err(|e| format!("failed to parse config at '{}': {e}", path.display()))
        },
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            tracing::debug!("config not found at '{}', using default values", path.display());

            Ok(Config::default())
        },
        Err(err) => {
            Err(format!("failed to read config at '{}': {err}", path.display()))
        },
    }
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT_CONFIG: &str = r#"
        [mpd]
        host = "localhost"
        port = 6600

        [server]
        bind = "0.0.0.0"
        port = 8989

        [logging]
        level = "info"
    "#;

    const CUSTOM_CONFIG: &str = r#"
        [mpd]
        host = "10.0.0.1"
        port = 6601
        password = "qwerty"

        [logging]
        level = "trace"
    "#;

    #[test]
    fn should_parse_empty_config() {
        let result = toml::from_str::<Config>("").unwrap();

        assert_eq!(result, Config::default());
    }

    #[test]
    fn should_parse_default_config() {
        let result = toml::from_str::<Config>(DEFAULT_CONFIG).unwrap();

        assert_eq!(result, Config::default());
    }

    #[test]
    fn should_parse_custom_config() {
        let result = toml::from_str::<Config>(CUSTOM_CONFIG).unwrap();

        assert_eq!(result, Config {
            mpd: Mpd {
                host: "10.0.0.1".to_owned(),
                port: 6601,
                password: Some("qwerty".to_owned()),
            },
            logging: Logging {
                level: Level::TRACE,
            },
            ..Config::default()
        });
    }
}
