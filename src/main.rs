extern crate clap;
use clap::{App, Arg, SubCommand};

mod commands;

fn main() {
    let matches = App::new("rdBrowser")
        .version("0.1")
        .author("Dmitry Danilov")
        .about("CLI Browser for RocksDB\nhttps://github.com/f-squirrel/rdbrowser")
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
                    Arg::with_name("KEY")
                        .help("Value to put")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();
    let cmd = commands::create(matches);
    cmd.run();
}
