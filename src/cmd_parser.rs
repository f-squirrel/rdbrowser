use super::command::{batchput, checkconsistency, delete, deleterange, get, put, scan};
use crate::command::traits::Command;
use clap::{App, AppSettings, Arg, ArgMatches};

pub fn build_cmd_args<'a>() -> ArgMatches<'a> {
    App::new(env!("CARGO_PKG_NAME"))
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
        .arg(
            Arg::with_name("column_family")
                .long("column_family")
                .help("Name of the column family to operate on")
                .required(false)
                .takes_value(true)
                .default_value(rocksdb::DEFAULT_COLUMN_FAMILY_NAME),
        )
        .subcommand(put::Put::args())
        .subcommand(get::Get::args())
        .subcommand(delete::Delete::args())
        .subcommand(batchput::BatchPut::args())
        .subcommand(scan::Scan::args())
        .subcommand(deleterange::DeleteRange::args())
        .subcommand(checkconsistency::CheckConsistency::args())
        .get_matches()
}
