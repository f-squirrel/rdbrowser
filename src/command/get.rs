use crate::command::traits::Command;
use crate::utils;
use clap::{App, Arg, ArgMatches, SubCommand};
use rocksdb::DB;
use std::boxed::Box;
use std::error::Error;

#[derive(Debug)]
pub struct Get<'a> {
    db: DB,
    key: &'a str,
    key_hex: bool,
    value_hex: bool,
}

impl<'a> Get<'a> {
    pub fn create(matches: &'a ArgMatches<'a>) -> Result<Box<dyn Command + 'a>, Box<dyn Error>> {
        let opts = Self::build_options(matches);
        let db = DB::open_cf(
            &opts,
            matches.value_of("db").unwrap(),
            &[matches.value_of("column_family").unwrap()],
        )?;
        let subcommand_matches = matches.subcommand_matches(Self::name()).unwrap();
        Ok(std::boxed::Box::new(Get {
            db,
            key: subcommand_matches.value_of("KEY").unwrap(),
            key_hex: subcommand_matches.is_present("key_hex")
                || subcommand_matches.is_present("hex"),
            value_hex: subcommand_matches.is_present("value_hex")
                || subcommand_matches.is_present("hex"),
        }))
    }
}

impl<'a> Command for Get<'a> {
    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let key = if self.key_hex {
            utils::hex::decode(&self.key)?
        } else {
            self.key.as_bytes().into()
        };
        match self.db.get(key)? {
            None => {
                eprintln!("Not Found");
            }
            Some(value) => {
                let output = if self.value_hex {
                    utils::hex::encode(value)
                } else {
                    String::from_utf8(value).unwrap()
                };
                println!("{}", output);
            }
        }
        Ok(())
    }

    fn args() -> App<'static, 'static> {
        SubCommand::with_name(Self::name())
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

    fn name() -> &'static str {
        "get"
    }
}
