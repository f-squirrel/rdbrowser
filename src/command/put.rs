use crate::command::traits::Command;
use clap::ArgMatches;
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
            Ok(_) => {}
            Err(error) => {
                panic!(
                    "Failed to put key: {} value: {}, error: {}",
                    self.key, self.value, error
                );
            }
        };
    }
}
