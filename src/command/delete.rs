use crate::command::traits::Command;
use clap::ArgMatches;
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
            Ok(_) => { println!("OK"); }
            Err(error) => {
                panic!("Failed to delete key: {} , error: {}", self.key, error);
            }
        };
    }
}
