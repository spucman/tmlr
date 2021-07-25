use crate::settings::error::ConfigurationError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("An error occurred while calling {0} with error: {1}")]
    TimeularApiError(String, String),
    #[error("Unable to parse response while {0}")]
    ParseJsonError(String),
    #[error("No authentication data found")]
    AuthenticationInformationMissingError,
    #[error("The given command was not found")]
    InvalidCommandError,
    #[error("Couldn't work with configuration file: {0}")]
    ConfigError(#[from] ConfigurationError),
    #[error("Couldn't determine default space")]
    NoDefaultSpaceFound,
    #[error("Couldn't parse date/time from string: {0}")]
    ParseChronoError(#[from] chrono::ParseError),
}
