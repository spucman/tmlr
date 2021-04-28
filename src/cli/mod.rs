use crate::{cli::config as cli_config, settings::Settings};
use clap::{App, Arg, SubCommand};

const VERSION: &str = "0.1.0";

const CMD_LIST: &str = "list";
const CMD_START: &str = "start";
const CMD_STOP: &str = "stop";
const CMD_CREATE: &str = "list";
const CMD_DELETE: &str = "delete";
const CMD_TRACKING: &str = "tracking";
const CMD_TIME_ENTRY: &str = "time-entry";
const CMD_ACTIVITY: &str = "activity";
const CMD_TAG: &str = "tag";
const CMD_MENTION: &str = "mention";

const ARG_CONFIG: &str = "config";

mod config;
mod create;
mod delete;
mod list;

pub fn create_cli() {
    let matches = App::new("tmlr")
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
            Arg::with_name("apikey")
                .help("Sets an API key")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("apisecret")
                .help("Sets an API secret")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
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
        )
        .get_matches();

    let cfg = Settings::new(matches.value_of(ARG_CONFIG));
}
