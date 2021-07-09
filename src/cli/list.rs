use super::{CMD_ACTIVITY, CMD_MENTION, CMD_TAG};
use crate::Result;
use clap::{App, ArgMatches, SubCommand};

pub const CMD_LIST: &str = "list";
pub const CMD_SPACE: &str = "space";

pub fn create_commands<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD_LIST)
        .about("Lists Resources")
        .subcommand(SubCommand::with_name(CMD_ACTIVITY).about("Lists all Activities"))
        .subcommand(SubCommand::with_name(CMD_TAG).about("Lists all Tags"))
        .subcommand(SubCommand::with_name(CMD_MENTION).about("Lists all Mentions"))
        .subcommand(SubCommand::with_name(CMD_SPACE).about("Lists all Spaces"))
}

pub fn handle_match<'a>(_matches: &ArgMatches<'a>) -> Result<()> {
    log::info!("Command {} not implemented", CMD_LIST);
    Ok(())
}
