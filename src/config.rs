//! A module to handle config parsing.

use serde::Deserialize;
use std::{fs::File, io::Read};
use thiserror::Error;

pub fn load(path: &str) -> Result<Config, ConfigError> {
    let mut s = String::new();
    let mut file = File::open(path).map_err(|err| ConfigError::FileNotFound(path.to_string(), err))?;
    file.read_to_string(&mut s);
    let config: Config = serde_json::from_str(&s).map_err(|err| ConfigError::JsonParseFailed(path.to_string(), err))?;

    Ok(config)
}

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub upstream: String,
    pub server: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            // TODO: Break this up more.
            // The http_reverse_proxy server requires 'http://' or 'https://'
            upstream: "http://127.0.0.1:8080".to_string(),
            server: "127.0.0.1:8000".to_string(),
        }
    }
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("file was not found at path {0}, cause: {1}")]
    FileNotFound(String, std::io::Error),

    #[error("file could not be parsed at path {0}, cause: {1}")]
    JsonParseFailed(String, serde_json::Error),
}
