use super::command::{delete, get, put};
use crate::command::traits::Command;
use clap::{App, AppSettings, Arg, ArgMatches};

pub fn get_cmd_args() -> ArgMatches<'static> {
    App::new("rdBrowser")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(
            format!(
                "{}\n{}",
                env!("CARGO_PKG_DESCRIPTION"),
                env!("CARGO_PKG_HOMEPAGE")
            )
            .as_str(),
        )
        .arg(
            Arg::with_name("db")
                .long("db")
                .value_name("PATH")
                .help("Sets path to the target database")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("create_if_missing")
                .long("create_if_missing")
                .help("Creates DB if misssing")
                .required(false)
                .takes_value(false),
        )
        .subcommand(put::Put::args())
        .subcommand(get::Get::args())
        .subcommand(delete::Delete::args())
        .get_matches()
}
