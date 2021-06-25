use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigurationError {
    #[error("Unable to find file: {0}")]
    FileNotFoundError(String),
    #[error("Unable to create file: {0}")]
    FileCreationError(String),
    #[error("Unable to write file: {0}")]
    FileWriteError(String),
    #[error("Unable to create directory: {0}")]
    DirCreationError(String),
    #[error("{0}")]
    MessageError(String),
    #[error("Unalbe to serialize toml: {0}")]
    TomlError(#[from] toml::ser::Error),
}
