use super::{
    config::{add_activity_alias, add_tag_alias},
    ARG_ALIAS, ARG_CONFIG, ARG_SPACE_ID, CMD_ACTIVITY, CMD_MENTION, CMD_TAG, CMD_TIME_ENTRY,
};
use crate::{error::Error::InvalidCommandError, timeular::Timeular, Result};
use clap::{App, Arg, ArgMatches, SubCommand};

pub const CMD_CREATE: &str = "create";
pub const ARG_ACTIVITY_NAME: &str = "name";
pub const ARG_ACTIVITY_COLOR: &str = "color";
pub const ARG_LABEL: &str = "label";
pub const ARG_KEY: &str = "key";

pub fn create_commands<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD_CREATE)
        .about("Create Resources")
        .subcommand(
            SubCommand::with_name(CMD_ACTIVITY)
                .alias("ac")
                .about("Creates an activity")
                .arg(
                    Arg::with_name(ARG_ACTIVITY_NAME)
                        .help("Defines the activity name")
                        .required(true)
                )
                .arg(
                    Arg::with_name(ARG_SPACE_ID)
                        .help("Defines the space where the activity should be created. If no space id is passed the default (private) space will be taken.")
                        .long(ARG_SPACE_ID)
                        .short("s")
                        .takes_value(true)
                        .required(false)
                )
                .arg(
                    Arg::with_name(ARG_ACTIVITY_COLOR)
                        .help("Defines the color the activity should have in the UI clients. If no color will be provided a random one will be generated.")
                        .long(ARG_ACTIVITY_COLOR)
                        .short("c")
                        .takes_value(true)
                        .required(false)
                )
                .arg(
                    Arg::with_name(ARG_ALIAS)
                        .help("Defines an alias for the newly created activity")
                        .long(ARG_ALIAS)
                        .short("a")
                        .takes_value(true)
                        .required(false)
                )
        )
        .subcommand(SubCommand::with_name(CMD_MENTION).about("Creates a mention"))
        .subcommand(
            SubCommand::with_name(CMD_TAG)
                .about("Creates a tag")
                .arg(
                    Arg::with_name(ARG_LABEL)
                        .help("Defines an alias for the newly created tag")
                        .required(true)
                )
                .arg(
                    Arg::with_name(ARG_KEY)
                        .help("Defines a key for the given tag. If no key is provided one will automatically generated.")
                        .long(ARG_KEY)
                        .short("k")
                        .takes_value(true)
                        .required(false)
                )
                .arg(
                    Arg::with_name(ARG_SPACE_ID)
                        .help("Defines the space where the activity should be created. If no space id is passed the default (private) space will be taken.")
                        .long(ARG_SPACE_ID)
                        .short("s")
                        .takes_value(true)
                        .required(false)
                )
                .arg(
                    Arg::with_name(ARG_ALIAS)
                        .help("Defines an alias for the newly created tag")
                        .long(ARG_ALIAS)
                        .short("a")
                        .takes_value(true)
                        .required(false)
                )
        )
        .subcommand(
            SubCommand::with_name(CMD_TIME_ENTRY)
                .alias("te")
                .about("Creates an time entry"),
        )
}

pub fn handle_match<'a>(matches: &ArgMatches<'a>, tmlr: &Timeular) -> Result<()> {
    if let (sub_cmd, Some(sub_matches)) = matches.subcommand() {
        match sub_cmd {
            CMD_ACTIVITY => handle_create_activity(tmlr, sub_matches),
            CMD_TAG => handle_create_tag(tmlr, sub_matches),
            CMD_TIME_ENTRY => {
                println!("Not implemented yet!");
                Ok(())
            }
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

fn handle_create_activity<'a>(tmlr: &Timeular, matches: &ArgMatches<'a>) -> Result<()> {
    let (ac_id, name) = tmlr.create_activity(
        matches
            .value_of(ARG_ACTIVITY_NAME)
            .map(|v| v.to_string())
            .expect("An activity name was provided"),
        matches.value_of(ARG_ACTIVITY_COLOR).map(|v| v.to_string()),
        matches.value_of(ARG_SPACE_ID).map(|v| v.to_string()),
    )?;
    log::info!("Activity \"{}\" was created.", name);

    if let Some(alias) = matches.value_of(ARG_ALIAS) {
        add_activity_alias(alias, &ac_id, matches.value_of(ARG_CONFIG))?;
    }

    Ok(())
}

fn handle_create_tag<'a>(tmlr: &Timeular, matches: &ArgMatches<'a>) -> Result<()> {
    let tag_id = tmlr.create_tag(
        matches
            .value_of(ARG_LABEL)
            .map(|v| v.to_string())
            .expect("An tag label was provided"),
        matches.value_of(ARG_KEY).map(|v| v.to_string()),
        matches.value_of(ARG_SPACE_ID).map(|v| v.to_string()),
    )?;
    log::info!("Tag created.");

    if let Some(alias) = matches.value_of(ARG_ALIAS) {
        add_tag_alias(alias, &tag_id.to_string(), matches.value_of(ARG_CONFIG))?;
    }

    Ok(())
}
