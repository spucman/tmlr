use crate::{
    error::Error::InvalidCommandError,
    settings::{Authentication, Settings},
    Result,
};
use clap::{App, Arg, ArgMatches, SubCommand};

pub const CMD_CONFIG: &str = "config";
pub const ARG_API_KEY: &str = "apikey";
pub const ARG_API_SECRET: &str = "apisecret";
pub const ARG_CONFIG: &str = "config";

const CMD_AUTH: &str = "auth";

pub fn create_commands<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD_CONFIG).about("Config Area").subcommand(
        SubCommand::with_name(CMD_AUTH)
            .about("Permanently authenticates a user")
            .arg(
                Arg::with_name(ARG_API_KEY)
                    .help("Sets the API key")
                    .required(true),
            )
            .arg(
                Arg::with_name(ARG_API_SECRET)
                    .help("API secret")
                    .required(true),
            ),
    )
}

pub fn handle_match<'a>(matches: &ArgMatches<'a>) -> Result<()> {
    if let (sub_cmd, Some(sub_matches)) = matches.subcommand() {
        match sub_cmd {
            CMD_AUTH => handle_create_config(
                sub_matches
                    .value_of(ARG_API_KEY)
                    .expect("A required argument api key"),
                sub_matches
                    .value_of(ARG_API_SECRET)
                    .expect("A required argument api_secret"),
                sub_matches.value_of(ARG_CONFIG),
            ),
            _ => Err(InvalidCommandError),
        }
    } else {
        Err(InvalidCommandError)
    }
}

fn handle_create_config(api_key: &str, api_secret: &str, custom_path: Option<&str>) -> Result<()> {
    let cfg = Settings {
        auth: Some(Authentication {
            api_secret: Some(api_secret.to_owned()),
            api_key: Some(api_key.to_owned()),
        }),
    };

    cfg.save(custom_path)
}
