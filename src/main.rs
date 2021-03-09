use crate::{cmd_create::create_create_commands, cmd_list::create_list_commands};
use clap::{App, Arg, SubCommand};

const VERSION: &str = "0.1.0";
const CMD_LIST: &str = "list";
const CMD_START: &str = "start";
const CMD_STOP: &str = "stop";
const CMD_CREATE: &str = "list";
const CMD_TRACKING: &str = "tracking";
const CMD_TIME_ENTRY: &str = "time-entry";
const CMD_ACTIVITY: &str = "activity";
const CMD_TAG: &str = "tag";
const CMD_MENTION: &str = "mention";

mod cmd_create;
mod cmd_list;

fn main() {
    let _matches = App::new("tmlr")
        .version(VERSION)
        .about("Timular CLI Client")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(create_list_commands())
        .subcommand(create_create_commands())
        .subcommand(
            SubCommand::with_name(CMD_START)
                .about("Starts a Resource")
                .subcommand(SubCommand::with_name(CMD_TRACKING)),
        )
        .subcommand(
            SubCommand::with_name(CMD_STOP)
                .about("Starts a Resource")
                .subcommand(SubCommand::with_name(CMD_TRACKING)),
        )
        .get_matches();
}
