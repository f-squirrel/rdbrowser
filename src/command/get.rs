use crate::command::traits::Command;
use clap::{App, Arg, ArgMatches, SubCommand};
use hex;
use rocksdb::DB;

#[derive(Debug)]
pub struct Get {
    db: DB,
    key: String,
    key_hex: bool,
    value_hex: bool,
}

impl Get {
    pub fn new(db: DB, matches: &ArgMatches) -> Get {
        Get {
            db,
            key: matches.value_of("KEY").unwrap().into(),
            key_hex: matches.is_present("key_hex") || matches.is_present("hex"),
            value_hex: matches.is_present("value_hex") || matches.is_present("hex"),
        }
    }
}

impl Command for Get {
    fn run(&self) {
        let key = if self.key_hex {
            hex::decode(self.key.as_bytes()).unwrap()
        } else {
            self.key.clone().into_bytes()
        };
        match self.db.get(key) {
            Ok(None) => {
                println!("Not Found");
            }
            Ok(Some(content)) => {
                let output = if self.value_hex {
                    hex::encode(content)
                } else {
                    String::from_utf8(content).unwrap()
                };
                println!("{}", output);
            }
            Err(error) => {
                panic!("Failed to get key: {}, error: {}", self.key, error);
            }
        };
    }

    fn args() -> App<'static, 'static> {
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
                    .help("Key to get")
                    .required(true)
                    .index(1),
            )
    }
}
