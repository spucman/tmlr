use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("An error occurred while calling {0} with error: {1}")]
    TimeularApiError(String, String),
    #[error("Unable to parse response")]
    ParseJsonError,
}
