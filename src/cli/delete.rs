use super::{CMD_ACTIVITY, CMD_DELETE, CMD_MENTION, CMD_TAG, CMD_TIME_ENTRY};
use clap::{App, SubCommand};

pub fn create_commands<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD_DELETE)
        .about("Delete Resources")
        .subcommand(SubCommand::with_name(CMD_ACTIVITY))
        .subcommand(SubCommand::with_name(CMD_MENTION))
        .subcommand(SubCommand::with_name(CMD_TAG))
        .subcommand(SubCommand::with_name(CMD_TIME_ENTRY).alias("te"))
}
