mod cli;
mod error;
mod settings;
mod timeular;
mod util;

type Result<T> = std::result::Result<T, error::Error>;

fn main() {
    cli::create_cli();
}
