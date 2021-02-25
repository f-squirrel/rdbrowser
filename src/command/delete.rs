use crate::command::traits::Command;
use crate::utils;
use clap::{App, Arg, ArgMatches, SubCommand};
use rocksdb::DB;
use std::boxed::Box;
use std::error::Error;

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
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let k = if self.key_hex {
            utils::hex::decode(&self.key)?
        } else {
            self.key.as_bytes().into()
        };
        match self.db.delete(k) {
            Ok(_) => {
                println!("OK");
            }
            Err(error) => {
                panic!("Failed to delete key: {} , error: {}", self.key, error);
            }
        }
        Ok(())
    }

    fn args() -> App<'static, 'static> {
        SubCommand::with_name(Self::name())
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
            .into()
    }

    fn name() -> &'static str {
        "delete"
    }
}
