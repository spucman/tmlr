use super::{CMD_ACTIVITY, CMD_MENTION, CMD_TAG, CMD_TIME_ENTRY};
use crate::{error::Error::InvalidCommandError, timeular::Timeular, Result};
use clap::{App, ArgMatches, SubCommand};

pub const CMD_CREATE: &str = "create";

pub fn create_commands<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD_CREATE)
        .about("Create Resources")
        .subcommand(SubCommand::with_name(CMD_ACTIVITY).about("Creates an activity"))
        .subcommand(SubCommand::with_name(CMD_MENTION).about("Creates a mention"))
        .subcommand(SubCommand::with_name(CMD_TAG).about("Creates a tag"))
        .subcommand(
            SubCommand::with_name(CMD_TIME_ENTRY)
                .alias("te")
                .about("Creates an time entry"),
        )
}

pub fn handle_match<'a>(matches: &ArgMatches<'a>, tmlr: Timeular) -> Result<()> {
    if let (sub_cmd, Some(sub_matches)) = matches.subcommand() {
        match sub_cmd {
            CMD_ACTIVITY => handle_create_activity(tmlr),
            CMD_TAG => {
                println!("Not implemented yet!");
                Ok(())
            }
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

fn handle_create_activity(tmlr: Timeular) -> Result<()> {
    tmlr.create_activity();
    Ok(())
}
