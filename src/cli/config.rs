use crate::{
    cli::{CMD_ACTIVITY, CMD_TAG},
    error::Error::InvalidCommandError,
    settings::{Alias, Authentication, Settings},
    Result,
};
use clap::{App, Arg, ArgMatches, SubCommand};

pub const CMD_SET: &str = "set";
pub const CMD_CONFIG: &str = "config";

pub const ARG_API_KEY: &str = "apikey";
pub const ARG_API_SECRET: &str = "apisecret";
pub const ARG_CONFIG: &str = "config";

pub const CMD_ALIAS: &str = "alias";
pub const ARG_ID: &str = "id";
pub const ARG_ALIAS: &str = "alias";

const CMD_AUTH: &str = "auth";

pub fn create_commands<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD_CONFIG).about("Config Area").subcommand(
        SubCommand::with_name(CMD_SET)
            .about("Sets config  values")
            .subcommand(
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
            .subcommand(
                SubCommand::with_name(CMD_ALIAS)
                    .about("Creates an alias")
                    .subcommand(
                        SubCommand::with_name(CMD_ACTIVITY)
                            .about("Creates an alias for the activity")
                            .arg(
                                Arg::with_name(ARG_ID)
                                    .help("Sets the activity id")
                                    .required(true),
                            )
                            .arg(
                                Arg::with_name(ARG_ALIAS)
                                    .help("Sets the alias for the given activity")
                                    .required(true),
                            ),
                    ),
            ),
    )
}

pub fn handle_match<'a>(matches: &ArgMatches<'a>) -> Result<()> {
    if let (sub_cmd, Some(sub_matches)) = matches.subcommand() {
        match sub_cmd {
            CMD_SET => handle_match_set(sub_matches),
            _ => Err(InvalidCommandError),
        }
    } else {
        Err(InvalidCommandError)
    }
}

fn handle_match_set<'a>(matches: &ArgMatches<'a>) -> Result<()> {
    if let (sub_cmd, Some(sub_matches)) = matches.subcommand() {
        match sub_cmd {
            CMD_AUTH => set_auth_to_config(
                sub_matches
                    .value_of(ARG_API_KEY)
                    .expect("A required argument api key"),
                sub_matches
                    .value_of(ARG_API_SECRET)
                    .expect("A required argument api_secret"),
                sub_matches.value_of(ARG_CONFIG),
            ),
            CMD_ALIAS => handle_match_alias(sub_matches),
            _ => Err(InvalidCommandError),
        }
    } else {
        Err(InvalidCommandError)
    }
}

fn set_auth_to_config(api_key: &str, api_secret: &str, custom_path: Option<&str>) -> Result<()> {
    let mut cfg = Settings::new(custom_path)?;

    cfg.auth = Some(Authentication {
        api_secret: Some(api_secret.to_owned()),
        api_key: Some(api_key.to_owned()),
    });

    cfg.save(custom_path)
}

fn handle_match_alias<'a>(matches: &ArgMatches<'a>) -> Result<()> {
    if let (sub_cmd, Some(sub_matches)) = matches.subcommand() {
        match sub_cmd {
            CMD_ACTIVITY | CMD_TAG => add_alias_to_config(
                sub_cmd,
                sub_matches
                    .value_of(ARG_ID)
                    .expect("A required argument id"),
                sub_matches
                    .value_of(ARG_ALIAS)
                    .expect("A required argument alias"),
                sub_matches.value_of(ARG_CONFIG),
            ),
            _ => Err(InvalidCommandError),
        }
    } else {
        Err(InvalidCommandError)
    }
}

fn add_alias_to_config(
    _section: &str,
    _key: &str,
    _value: &str,
    custom_path: Option<&str>,
) -> Result<()> {
    let cfg = Settings::new(custom_path)?;

    //TODO create cfg here

    cfg.save(custom_path)
}
