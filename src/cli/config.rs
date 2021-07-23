use crate::{
    cli::{ARG_ALIAS, CMD_ACTIVITY, CMD_TAG},
    error::Error::InvalidCommandError,
    settings::{error::ConfigurationError::FileNotFoundError, Authentication, Settings},
    Result,
};
use clap::{App, Arg, ArgMatches, SubCommand};

pub const CMD_SET: &str = "set";
const CMD_DELETE: &str = "delete";
pub const CMD_CONFIG: &str = "config";

pub const ARG_API_KEY: &str = "apikey";
pub const ARG_API_SECRET: &str = "apisecret";
pub const ARG_CONFIG: &str = "config";

pub const CMD_ALIAS: &str = "alias";
pub const ARG_ID: &str = "id";

const CMD_AUTH: &str = "auth";

pub fn create_commands<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD_CONFIG)
        .about("Config Area")
        .subcommand(
            SubCommand::with_name(CMD_SET)
                .about("Sets config values")
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
                                .about("Creates an alias for an activity")
                                .arg(
                                    Arg::with_name(ARG_ID)
                                        .help("Sets the activity id")
                                        .required(true),
                                )
                                .arg(
                                    Arg::with_name(ARG_ALIAS)
                                        .help("Sets the alias for the given activity id")
                                        .required(true),
                                ),
                        )
                        .subcommand(
                            SubCommand::with_name(CMD_TAG)
                                .about("Creates an alias for a tag")
                                .arg(
                                    Arg::with_name(ARG_ID)
                                        .help("Sets the tag id")
                                        .required(true),
                                )
                                .arg(
                                    Arg::with_name(ARG_ALIAS)
                                        .help("Sets the alias for the given tag id"),
                                ),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_DELETE)
                .about("Deletes a config value")
                .subcommand(
                    SubCommand::with_name(CMD_ALIAS)
                        .about("Deletes an alias")
                        .subcommand(
                            SubCommand::with_name(CMD_ACTIVITY)
                                .about("Deletes an alias of an activity")
                                .arg(
                                    Arg::with_name(ARG_ALIAS)
                                        .help("Deletes the alias with the defined alias of an activity")
                                        .required(true),
                                ),
                        )
                        .subcommand(
                            SubCommand::with_name(CMD_TAG)
                                .about("Deletes an alias of a tag")
                                .arg(
                                    Arg::with_name(ARG_ALIAS)
                                        .help("Deletes an alias with the defined alias of a tag").required(true),
                                ),
                        ),
                ),
        )
}

pub fn handle_match<'a>(matches: &ArgMatches<'a>) -> Result<()> {
    if let (sub_cmd, Some(sub_matches)) = matches.subcommand() {
        match sub_cmd {
            CMD_SET => handle_match_set(sub_matches),
            CMD_DELETE => handle_match_delete(sub_matches),
            _ => {
                println!("{}", matches.usage());
                Err(InvalidCommandError)
            }
        }
    } else {
        println!("{}", matches.usage());
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
            CMD_ALIAS => handle_match_set_alias(sub_matches),
            _ => Err(InvalidCommandError),
        }
    } else {
        Err(InvalidCommandError)
    }
}

fn set_auth_to_config(api_key: &str, api_secret: &str, custom_path: Option<&str>) -> Result<()> {
    let mut cfg = load_settings(custom_path)?;

    cfg.auth = Some(Authentication {
        api_secret: Some(api_secret.to_owned()),
        api_key: Some(api_key.to_owned()),
    });

    cfg.save(custom_path).map_err(|e| e.into())
}

fn handle_match_set_alias<'a>(matches: &ArgMatches<'a>) -> Result<()> {
    if let (sub_cmd, Some(sub_matches)) = matches.subcommand() {
        match sub_cmd {
            CMD_ACTIVITY => add_activity_alias(
                sub_matches
                    .value_of(ARG_ALIAS)
                    .expect("A required argument alias"),
                sub_matches
                    .value_of(ARG_ID)
                    .expect("A required argument id"),
                sub_matches.value_of(ARG_CONFIG),
            ),
            CMD_TAG => add_tag_alias(
                sub_matches
                    .value_of(ARG_ALIAS)
                    .expect("A required argument alias"),
                sub_matches
                    .value_of(ARG_ID)
                    .expect("A required argument id"),
                sub_matches.value_of(ARG_CONFIG),
            ),
            _ => Err(InvalidCommandError),
        }
    } else {
        Err(InvalidCommandError)
    }
}

pub fn add_tag_alias(alias: &str, tag_id: &str, custom_path: Option<&str>) -> Result<()> {
    let mut cfg = load_settings(custom_path)?;
    cfg.add_tag_alias(alias.to_string(), tag_id.to_string());
    cfg.save(custom_path).map_err(|e| e.into())
}

pub fn add_activity_alias(alias: &str, activity_id: &str, custom_path: Option<&str>) -> Result<()> {
    let mut cfg = load_settings(custom_path)?;
    cfg.add_activity_alias(alias.to_string(), activity_id.to_string());
    match cfg.save(custom_path) {
        Ok(_) => {
            log::info!("Alias \"{}\" for activity was created.", alias);
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}

fn load_settings(custom_path: Option<&str>) -> Result<Settings> {
    let settings = Settings::new(custom_path);
    if custom_path.is_none() {
        match settings {
            Ok(v) => Ok(v),
            Err(err) => match err {
                FileNotFoundError(_) => Ok(Settings::default()),
                _ => Err(err.into()),
            },
        }
    } else {
        settings.map_err(|e| e.into())
    }
}

fn handle_match_delete<'a>(matches: &ArgMatches<'a>) -> Result<()> {
    if let (sub_cmd, Some(sub_matches)) = matches.subcommand() {
        match sub_cmd {
            CMD_ALIAS => handle_match_delete_alias(sub_matches),
            _ => Err(InvalidCommandError),
        }
    } else {
        Err(InvalidCommandError)
    }
}

fn handle_match_delete_alias<'a>(matches: &ArgMatches<'a>) -> Result<()> {
    if let (sub_cmd, Some(sub_matches)) = matches.subcommand() {
        match sub_cmd {
            CMD_ACTIVITY => delete_activity_alias(
                sub_matches
                    .value_of(ARG_ALIAS)
                    .expect("A required argument alias"),
                sub_matches.value_of(ARG_CONFIG),
            ),
            CMD_TAG => delete_tag_alias(
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

fn delete_tag_alias(alias: &str, custom_path: Option<&str>) -> Result<()> {
    let mut cfg = load_settings(custom_path)?;
    cfg.remove_tag_alias(alias);
    cfg.save(custom_path).map_err(|e| e.into())
}

fn delete_activity_alias(alias: &str, custom_path: Option<&str>) -> Result<()> {
    let mut cfg = load_settings(custom_path)?;
    cfg.remove_activity_alias(alias);
    cfg.save(custom_path).map_err(|e| e.into())
}
