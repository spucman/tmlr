use super::{CMD_ACTIVITY, CMD_MENTION, CMD_TAG, CMD_TIME_ENTRY};
use clap::{App, SubCommand};

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

pub fn handle_match() {
    log::info!("Command {} not implemented", CMD_CREATE);
}
