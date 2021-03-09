use super::{CMD_ACTIVITY, CMD_LIST, CMD_MENTION, CMD_TAG};
use clap::{App, SubCommand};

pub fn create_list_commands<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD_LIST)
        .about("Lists Resources")
        .subcommand(SubCommand::with_name(CMD_ACTIVITY).about("Lists all Activities"))
        .subcommand(SubCommand::with_name(CMD_TAG).about("Lists all Tags"))
        .subcommand(SubCommand::with_name(CMD_MENTION).about("Lists all Mentions"))
}
