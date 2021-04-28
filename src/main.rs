mod cli;
mod error;
mod settings;
mod timeular;

type Result<T> = std::result::Result<T, error::Error>;

fn main() {
    cli::create_cli();
}
