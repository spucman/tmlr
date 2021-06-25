use super::{CMD_ACTIVITY, CMD_MENTION, CMD_TAG, CMD_TIME_ENTRY};
use crate::Result;
use clap::{App, SubCommand};

pub const CMD_DELETE: &str = "delete";

pub fn create_commands<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD_DELETE)
        .about("Delete Resources")
        .subcommand(SubCommand::with_name(CMD_ACTIVITY).about("Deletes an activity"))
        .subcommand(SubCommand::with_name(CMD_MENTION).about("Deletes a mention"))
        .subcommand(SubCommand::with_name(CMD_TAG).about("Deletes a tag"))
        .subcommand(
            SubCommand::with_name(CMD_TIME_ENTRY)
                .alias("te")
                .about("Deletes a time entry"),
        )
}

pub fn handle_match() -> Result<()> {
    log::info!("Command {} not implemented", CMD_DELETE);
    Ok(())
}
