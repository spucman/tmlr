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
}
