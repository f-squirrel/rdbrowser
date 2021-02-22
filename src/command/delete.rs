use crate::command::traits::Command;
use clap::{App, Arg, ArgMatches, SubCommand};
use hex;
use rocksdb::DB;

#[derive(Debug)]
pub struct Delete {
    db: DB,
    key: String,
    key_hex: bool,
}

impl Delete {
    pub fn new(db: DB, matches: &ArgMatches) -> Delete {
        Delete {
            db,
            key: matches.value_of("KEY").unwrap().into(),
            key_hex: matches.is_present("key_hex") || matches.is_present("hex"),
        }
    }
}

impl Command for Delete {
    fn run(&self) {
        let key = if self.key_hex {
            hex::decode(self.key.as_bytes()).unwrap()
        } else {
            self.key.clone().into_bytes()
        };
        match self.db.delete(key) {
            Ok(_) => {
                println!("OK");
            }
            Err(error) => {
                panic!("Failed to delete key: {} , error: {}", self.key, error);
            }
        };
    }

    fn args() -> App<'static, 'static> {
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
            )
    }
}
