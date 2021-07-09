use crate::{
    cli::config::{self as cli_config, ARG_API_KEY, ARG_API_SECRET, ARG_CONFIG},
    error::Error::{AuthenticationInformationMissingError, InvalidCommandError},
    settings::Settings,
    timeular::{Timeular, TimeularAuth, TimeularCredentials},
    Result,
};
use clap::{App, Arg, SubCommand};

const VERSION: &str = "0.1.0";

const CMD_START: &str = "start";
const CMD_STOP: &str = "stop";
const CMD_TRACKING: &str = "tracking";
const CMD_TIME_ENTRY: &str = "time-entry";
const CMD_ACTIVITY: &str = "activity";
const CMD_TAG: &str = "tag";
const CMD_MENTION: &str = "mention";

const ARG_VERBOSE: &str = "verbose";
const ARG_SPACE_ID: &str = "spaceId";
const ARG_ALIAS: &str = "alias";

mod config;
mod create;
mod delete;
mod list;

pub fn create_cli() -> Result<()> {
    let app = App::new("tmlr")
        .version(VERSION)
        .about("Timular CLI Client")
        .arg(
            Arg::with_name(ARG_CONFIG)
                .short("c")
                .long(ARG_CONFIG)
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(ARG_API_KEY)
                .long(ARG_API_KEY)
                .help("Sets an API key")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name(ARG_API_SECRET)
                .long(ARG_API_SECRET)
                .help("Sets an API secret")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name(ARG_VERBOSE)
                .long(ARG_VERBOSE)
                .short("v")
                .help("Sets the level of verbosity"),
        )
        .subcommand(list::create_commands())
        .subcommand(create::create_commands())
        .subcommand(delete::create_commands())
        .subcommand(cli_config::create_commands())
        .subcommand(
            SubCommand::with_name(CMD_START)
                .about("Starts a Resource")
                .subcommand(SubCommand::with_name(CMD_TRACKING)),
        )
        .subcommand(
            SubCommand::with_name(CMD_STOP)
                .about("Starts a Resource")
                .subcommand(SubCommand::with_name(CMD_TRACKING)),
        );
    let matches = app.clone().get_matches();

    crate::util::logging::init(matches.is_present(ARG_VERBOSE));

    if let Some(sub_matches) = matches.subcommand_matches(config::CMD_CONFIG) {
        return config::handle_match(sub_matches);
    }

    let cfg = match Settings::new(matches.value_of(ARG_CONFIG)) {
        Ok(v) => Some(v),
        Err(_) => {
            log::debug!("No cfg found.");
            None
        }
    };

    match matches.subcommand() {
        (sub_cmd, Some(sub_matches)) => {
            let auth = create_auth_data(
                cfg.as_ref(),
                matches.value_of(ARG_API_KEY),
                matches.value_of(ARG_API_SECRET),
            );

            if auth.is_none() {
                return Err(AuthenticationInformationMissingError);
            }

            let tmlr = Timeular::new(auth.expect("Auth data found"))?;

            match sub_cmd {
                list::CMD_LIST => list::handle_match(sub_matches),
                create::CMD_CREATE => create::handle_match(sub_matches, &tmlr),
                delete::CMD_DELETE => delete::handle_match(),
                CMD_START => {
                    log::info!("Not implemented");
                    Ok(())
                }
                CMD_STOP => {
                    log::info!("Not implemented");
                    Ok(())
                }
                _ => {
                    log::info!("Nothing found{}", matches.usage());
                    Err(InvalidCommandError)
                }
            }
        }
        _ => {
            app.write_help(&mut std::io::stdout())
                .expect("Failed to write help");

            Err(InvalidCommandError)
        }
    }
}

fn create_auth_data(
    cfg: Option<&Settings>,
    api_key: Option<&str>,
    api_secret: Option<&str>,
) -> Option<TimeularAuth> {
    let auth = match cfg {
        Some(c) => {
            if let Some(auth_cfg) = &c.auth {
                match (&auth_cfg.api_key, &auth_cfg.api_secret) {
                    (Some(key), Some(secret)) => {
                        Some(TimeularAuth::new(key.to_owned(), secret.to_owned()))
                    }
                    _ => None,
                }
            } else {
                None
            }
        }
        None => None,
    };

    if let (Some(key), Some(secret)) = (api_key, api_secret) {
        if let Some(a) = auth {
            Some(TimeularAuth {
                credentials: TimeularCredentials {
                    api_key: key.to_string(),
                    api_secret: secret.to_string(),
                },
                token: a.token,
            })
        } else {
            Some(TimeularAuth::new(key.to_string(), secret.to_string()))
        }
    } else {
        auth
    }
}

#[cfg(test)]
mod tests {
    use super::create_auth_data;
    use crate::{
        settings::{Authentication, Settings},
        timeular::TimeularAuth,
    };

    #[test]
    fn test_create_auth_data() {
        // No Settings
        do_test_create_auth_data(None, None);

        // Empty Settings
        do_test_create_auth_data(
            Some(&Settings {
                auth: None,
                alias: None,
            }),
            None,
        );

        // Settings with Authentication Section only
        do_test_create_auth_data(
            Some(&Settings {
                auth: Some(Authentication {
                    api_key: None,
                    api_secret: None,
                }),
                alias: None,
            }),
            None,
        );

        // Settings with api_key only
        do_test_create_auth_data(
            Some(&Settings {
                auth: Some(Authentication {
                    api_key: Some("key".to_owned()),
                    api_secret: None,
                }),
                alias: None,
            }),
            None,
        );

        // Settings with api_secret only
        do_test_create_auth_data(
            Some(&Settings {
                auth: Some(Authentication {
                    api_key: None,
                    api_secret: Some("secret".to_owned()),
                }),
                alias: None,
            }),
            None,
        );

        // Settings with api_key and api_secret
        do_test_create_auth_data(
            Some(&Settings {
                auth: Some(Authentication {
                    api_key: Some("key".to_owned()),
                    api_secret: Some("secret".to_owned()),
                }),
                alias: None,
            }),
            Some(TimeularAuth::new("key".to_owned(), "secret".to_owned())),
        );
    }

    fn do_test_create_auth_data(
        settings: Option<&Settings>,
        default_auth_data: Option<TimeularAuth>,
    ) {
        let api_key = Some("some_api_key");
        let api_secret = Some("some_api_secret");
        let auth_data = Some(TimeularAuth::new(
            api_key.expect("value is there").to_owned(),
            api_secret.expect("value is there").to_owned(),
        ));

        assert!(matches!(
            create_auth_data(settings, None, None),
            default_auth_data
        ));
        assert!(matches!(
            create_auth_data(settings, None, api_secret),
            default_auth_data
        ));
        assert!(matches!(
            create_auth_data(settings, api_key, None),
            default_auth_data
        ));
        assert!(matches!(
            create_auth_data(None, api_key, api_secret),
            auth_data
        ));
    }
}
