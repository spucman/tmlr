use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("An error occurred while calling {0} with error: {1}")]
    TimeularApiError(String, String),
    #[error("Unable to parse response")]
    ParseJsonError,
    #[error("No authentication data found")]
    AuthenticationInformationMissingError,
    #[error("The given command was not found")]
    InvalidCommandError,
    #[error("Unable to load config: {0}")]
    ConfigurationError(String),
    #[error("Unalbe to serialize toml: {0}")]
    TomlError(#[from] toml::ser::Error),
    #[error("Unable to create file: {0}")]
    FileCreationError(String),
    #[error("Unable to write file: {0}")]
    FileWriteError(String),
    #[error("Unable to create directory: {0}")]
    DirCreationError(String),
}
