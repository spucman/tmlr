use clap::{App, Arg, SubCommand};

pub const CMD_CONFIG: &str = "config";
pub const ARG_API_KEY: &str = "apikey";
pub const ARG_API_SECRET: &str = "apisecret";

pub fn create_commands<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD_CONFIG)
        .about("Config Area")
        .subcommand(SubCommand::with_name("auth").about("Permanently authenticates a user"))
        .arg(
            Arg::with_name(ARG_API_KEY)
                .help("Sets the API key")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name(ARG_API_SECRET)
                .help("API secret")
                .required(true)
                .takes_value(true),
        )
}

pub fn handle_match() {
    log::info!("Command {} not implemented", CMD_CONFIG);
}
