use crate::command::traits::Command;
use clap::{App, Arg, ArgMatches, SubCommand};
use hex;
use rocksdb::DB;

#[derive(Debug)]
pub struct Put {
    db: DB,
    key: String,
    value: String,
    key_hex: bool,
    value_hex: bool,
}

impl Put {
    pub fn new(db: DB, matches: &ArgMatches) -> Put {
        Put {
            db,
            key: matches.value_of("KEY").unwrap().into(),
            value: matches.value_of("VALUE").unwrap().into(),
            key_hex: matches.is_present("key_hex") || matches.is_present("hex"),
            value_hex: matches.is_present("value_hex") || matches.is_present("hex"),
        }
    }
}

impl Command for Put {
    fn run(&self) {
        let key = if self.key_hex {
            hex::decode(self.key.as_bytes()).unwrap()
        } else {
            self.key.clone().into_bytes()
        };
        let value = if self.value_hex {
            hex::decode(self.value.as_bytes()).unwrap()
        } else {
            self.value.clone().into_bytes()
        };
        match self.db.put(key, value) {
            Ok(_) => {
                println!("OK");
            }
            Err(error) => {
                panic!(
                    "Failed to put key: {} value: {}, error: {}",
                    self.key, self.value, error
                );
            }
        };
    }

    fn args() -> App<'static, 'static> {
        SubCommand::with_name(Self::name())
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
            )
    }

    fn name() -> &'static str {
        "put"
    }
}
