use crate::command::traits::Command;
use crate::utils;
use clap::{App, Arg, ArgMatches, SubCommand};
use rocksdb::{Direction, IteratorMode, DB};
use std::boxed::Box;
use std::cmp;
use std::error::Error;
use std::option::Option;

fn compare(first: &[u8], second: &[u8]) -> i32 {
    for i in 0..cmp::min(first.len(), second.len()) {
        //
        if first[i] < second[i] {
            return -1;
        }
        if first[i] > second[i] {
            return 1;
        }
    }
    0
}

#[derive(Debug)]
pub struct Scan<'a> {
    db: DB,
    from: Option<&'a str>,
    to: Option<&'a str>,
    key_hex: bool,
    value_hex: bool,
}

impl<'a> Scan<'a> {
    pub fn new(matches: &'a ArgMatches<'a>) -> Result<Box<dyn Command + 'a>, Box<dyn Error>> {
        let opts = Self::build_options(matches);
        let db = DB::open_cf(
            &opts,
            matches.value_of("db").unwrap(),
            &[matches.value_of("column_family").unwrap()],
        )?;
        let subcommand_matches = matches.subcommand_matches(Self::name()).unwrap();
        Ok(std::boxed::Box::new(Scan {
            db,
            from: subcommand_matches.value_of("from"),
            to: subcommand_matches.value_of("to"),
            key_hex: subcommand_matches.is_present("key_hex")
                || subcommand_matches.is_present("hex"),
            value_hex: subcommand_matches.is_present("value_hex")
                || subcommand_matches.is_present("hex"),
        }))
    }
}

impl<'a> Command for Scan<'a> {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let iter = match self.from {
            None => self.db.iterator(IteratorMode::Start),
            Some(from) => {
                let f = if self.key_hex {
                    utils::hex::decode(from)?
                } else {
                    Vec::from(from.as_bytes())
                };
                self.db
                    .iterator(IteratorMode::From(f.as_ref(), Direction::Forward))
            }
        };
        let end = match self.to {
            None => None,
            Some(to) => {
                if self.key_hex {
                    Some(utils::hex::decode(to)?)
                } else {
                    Some(Vec::from(to.as_bytes()))
                }
            }
        };
        for (key, value) in iter {
            match end {
                None => {}
                Some(ref e) => {
                    if compare(key.as_ref(), e.as_ref()) >= 0 {
                        break;
                    }
                }
            }
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
            .arg(
                Arg::with_name("from")
                    .long("from")
                    .help("key to scan from")
                    .required(false)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("to")
                    .long("to")
                    .help("key to scan to")
                    .required(false)
                    .takes_value(true),
            )
    }

    fn name() -> &'static str {
        "scan"
    }
}
