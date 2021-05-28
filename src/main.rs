use error::Error;
use std::result::Result as StdResult;

mod cli;
mod error;
mod settings;
mod timeular;
mod util;

type Result<T> = StdResult<T, Error>;

fn main() -> Result<()> {
    let result = cli::create_cli();

    if let Err(err) = result.as_ref() {
        if err != &Error::InvalidCommandError {
            log::error!("{}", err.to_string());
        }
    }

    result
}
