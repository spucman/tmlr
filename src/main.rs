use error::Error;
use std::{process, result::Result as StdResult};

mod cli;
mod error;
mod settings;
mod timeular;
mod util;

type Result<T> = StdResult<T, Error>;

fn main() {
    match cli::create_cli() {
        Ok(()) => {}
        Err(err) => match err {
            Error::InvalidCommandError => process::exit(1),
            _ => {
                log::error!("{}", err.to_string());
                process::exit(1)
            }
        },
    }
    /*
    if let Err(err) = cli::create_cli() {
        if err != Error::InvalidCommandError {
            log::error!("{}", err.to_string());
        }
        process::exit(1);
    }
    */
}
