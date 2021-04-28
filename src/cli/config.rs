use clap::{App, Arg, SubCommand};

const CMD_CONFIG: &str = "config";
const ARG_API_KEY: &str = "apikey";
const ARG_API_SECRET: &str = "apisecret";

pub fn create_commands<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD_CONFIG)
        .about("Config Area")
        .subcommand(SubCommand::with_name("auth").about("Permanently authenticates a user"))
        .arg(
            Arg::with_name(ARG_API_KEY)
                .help("Sets the current ")
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
