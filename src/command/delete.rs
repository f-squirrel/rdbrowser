use crate::command::traits::Command;
use crate::utils;
use clap::{App, Arg, ArgMatches, SubCommand};
use rocksdb::DB;
use std::boxed::Box;
use std::error::Error;

#[derive(Debug)]
pub struct Delete<'a> {
    db: DB,
    key: &'a str,
    key_hex: bool,
}

impl<'a> Delete<'a> {
    pub fn create(matches: &'a ArgMatches<'a>) -> Result<Box<dyn Command + 'a>, Box<dyn Error>> {
        let opts = Self::build_options(matches);
        let db = DB::open_cf(
            &opts,
            matches.value_of("db").unwrap(),
            &[matches.value_of("column_family").unwrap()],
        )?;
        let subcommand_matches = matches.subcommand_matches(Self::name()).unwrap();
        Ok(std::boxed::Box::new(Delete {
            db,
            key: subcommand_matches.value_of("KEY").unwrap(),
            key_hex: subcommand_matches.is_present("key_hex")
                || subcommand_matches.is_present("hex"),
        }))
    }
}

impl<'a> Command for Delete<'a> {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let k = if self.key_hex {
            utils::hex::decode(&self.key)?
        } else {
            self.key.as_bytes().into()
        };
        self.db.delete(k)?;
        println!("OK");
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
    }

    fn name() -> &'static str {
        "delete"
    }
}
