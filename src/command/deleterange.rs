use crate::command::traits::Command;
use crate::utils;
use clap::{App, Arg, ArgMatches, SubCommand};
use rocksdb::{DB, DEFAULT_COLUMN_FAMILY_NAME};
use std::boxed::Box;
use std::error::Error;

#[derive(Debug)]
pub struct DeleteRange<'a> {
    db: DB,
    from_key: &'a str,
    to_key: &'a str,
    key_hex: bool,
}

impl<'a> DeleteRange<'a> {
    pub fn new(db: DB, matches: &'a ArgMatches<'a>) -> DeleteRange<'a> {
        DeleteRange {
            db,
            from_key: matches.value_of("BEGIN KEY").unwrap(),
            to_key: matches.value_of("END KEY").unwrap(),
            key_hex: matches.is_present("key_hex") || matches.is_present("hex"),
        }
    }
}

impl<'a> Command for DeleteRange<'a> {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let (from, to) = if self.key_hex {
            (
                utils::hex::decode(&self.from_key)?,
                utils::hex::decode(&self.to_key)?,
            )
        } else {
            (
                self.from_key.as_bytes().into(),
                self.to_key.as_bytes().into(),
            )
        };
        self.db.delete_range_cf(
            self.db.cf_handle(DEFAULT_COLUMN_FAMILY_NAME).unwrap(),
            from,
            to,
        )?;
        println!("OK");
        Ok(())
    }

    fn args() -> App<'static, 'static> {
        SubCommand::with_name(Self::name())
            .about("Delete range from <BEGIN KEY> to <END KEY>")
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
                Arg::with_name("BEGIN KEY")
                    .help("Delete from this key")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::with_name("END KEY")
                    .help("Delete until this key")
                    .required(true)
                    .index(2),
            )
            .into()
    }

    fn name() -> &'static str {
        "deleterange"
    }
}
