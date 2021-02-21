use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

pub fn get_cmd_args() -> ArgMatches<'static> {
    App::new("rdBrowser")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
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
        .subcommand(
            SubCommand::with_name("put")
                .about("Puts given key value to the DB")
                .arg(
                    Arg::with_name("hex")
                        .long("hex")
                        .help("Key and value provided in hex format")
                        .required(false)
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("value_hex")
                        .long("value_hex")
                        .help("Value provided in hex format")
                        .required(false)
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("key_hex")
                        .long("key_hex")
                        .help("Key provided in hex format")
                        .required(false)
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("KEY")
                        .help("Value to put")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("VALUE")
                        .help("Value to put")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Gets given key value to the DB")
                .arg(
                    Arg::with_name("hex")
                        .long("hex")
                        .help("Key and value provided in hex format")
                        .required(false)
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("value_hex")
                        .long("value_hex")
                        .help("Value provided in hex format")
                        .required(false)
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("key_hex")
                        .long("key_hex")
                        .help("Key provided in hex format")
                        .required(false)
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("KEY")
                        .help("Key to put")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Deletes given key from to the DB")
                .arg(
                    Arg::with_name("hex")
                        .long("hex")
                        .help("Key in hex format")
                        .required(false)
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("key_hex")
                        .long("key_hex")
                        .help("Key provided in hex format")
                        .required(false)
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("KEY")
                        .help("Key to delete")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches()
}
