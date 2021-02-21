use clap::ArgMatches;
use hex;
use rocksdb::{Options, DB};
use std::boxed::Box;

pub trait Command {
    fn run(&self);
}

pub fn create(matches: ArgMatches) -> Box<dyn Command> {
    let mut opts = Options::default();
    opts.create_if_missing(matches.is_present("create_if_missing"));
    //TODO(DD): handle in a more elegant way, i.e. not crash
    let db = DB::open(&opts, matches.value_of("db").unwrap()).unwrap();
    match matches.subcommand() {
        ("put", Some(put)) => Box::new(Put::new(db, put)),
        ("get", Some(get)) => Box::new(Get::new(db, get)),
        ("", None) => unreachable!(),
        _ => unreachable!(),
    }
}

#[derive(Debug)]
pub struct Put {
    db: DB,
    key: String,
    value: String,
    key_hex: bool,
    value_hex: bool,
}

impl Put {
    fn new(db: DB, matches: &ArgMatches) -> Put {
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

#[derive(Debug)]
pub struct Get {
    db: DB,
    key: String,
    key_hex: bool,
    value_hex: bool,
}

impl Get {
    fn new(db: DB, matches: &ArgMatches) -> Get {
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
                println!("Not Found???");
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
                panic!("Failed to put key: {}, error: {}", self.key, error);
            }
        };
    }
}
