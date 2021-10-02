use crate::settings::error::ConfigurationError;
use chrono::ParseError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("An error occurred while calling {0} with error: {1}")]
    TimeularApi(String, String),
    #[error("Unable to parse response while {0}")]
    ParseJson(String),
    #[error("No authentication data found")]
    AuthenticationInformationMissing,
    #[error("The given command was not found")]
    InvalidCommand,
    #[error("Couldn't work with configuration file: {0}")]
    Config(#[from] ConfigurationError),
    #[error("Couldn't determine default space")]
    NoDefaultSpaceFound,
    #[error("Couldn't parse date/time: {0}")]
    ParseDateTime(#[from] ParseError),
}
