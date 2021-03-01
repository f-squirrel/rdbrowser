use crate::command::traits::Command;
use crate::utils;
use clap::{App, Arg, ArgMatches, SubCommand};
use rocksdb::{IteratorMode, DB};
use std::boxed::Box;
use std::error::Error;

#[derive(Debug)]
pub struct Scan {
    db: DB,
    key_hex: bool,
    value_hex: bool,
}

impl Scan {
    pub fn new(db: DB, matches: &ArgMatches) -> Scan {
        Scan {
            db,
            key_hex: matches.is_present("key_hex") || matches.is_present("hex"),
            value_hex: matches.is_present("value_hex") || matches.is_present("hex"),
        }
    }
}

impl Command for Scan {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let iter = self.db.iterator(IteratorMode::Start);
        for (key, value) in iter {
            let k = if self.key_hex {
                utils::hex::encode(key.as_ref())
            } else {
                String::from_utf8_lossy(key.as_ref()).into()
            };
            let v = if self.value_hex {
                utils::hex::encode(value.as_ref())
            } else {
                String::from_utf8_lossy(value.as_ref()).into()
            };
            println!("{} : {}", k, v);
        }
        Ok(())
    }

    fn args() -> App<'static, 'static> {
        SubCommand::with_name(Self::name())
            .about("Prints key : value from the DB")
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
            .into()
    }

    fn name() -> &'static str {
        "scan"
    }
}
