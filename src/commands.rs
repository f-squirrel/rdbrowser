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
        ("put", Some(put)) => Box::new(Put {
            db,
            key: put.value_of("KEY").unwrap().into(),
            value: put.value_of("VALUE").unwrap().into(),
            is_hex: put.is_present("hex"),
        }),
        ("get", Some(get)) => Box::new(Get {
            db,
            key: get.value_of("KEY").unwrap().into(),
            is_hex: get.is_present("hex"),
        }),
        ("", None) => unreachable!(),
        _ => unreachable!(),
    }
}

#[derive(Debug)]
pub struct Put {
    db: DB,
    key: String,
    value: String,
    is_hex: bool,
}

impl Command for Put {
    fn run(&self) {
        let (key, value) = if self.is_hex {
            (
                hex::decode(self.key.as_bytes()).unwrap(),
                hex::decode(self.value.as_bytes()).unwrap(),
            )
        } else {
            (
                self.key.clone().into_bytes(),
                self.value.clone().into_bytes(),
            )
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
    is_hex: bool,
}

impl Command for Get {
    fn run(&self) {
        let key = if self.is_hex {
            hex::decode(self.key.as_bytes()).unwrap()
        } else {
            self.key.clone().into_bytes()
        };
        match self.db.get(key) {
            Ok(None) => {
                println!("Not Found???");
            }
            Ok(Some(content)) => {
                let output = if self.is_hex {
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
