use crate::command::traits::Command;
use crate::utils;
use clap::{App, Arg, ArgMatches, SubCommand};
use rocksdb::{WriteBatch, DB};
use simple_error::SimpleError;
use std::boxed::Box;
use std::error::Error;

#[derive(Debug)]
pub struct BatchPut<'a> {
    db: DB,
    key_values: Vec<&'a str>,
    key_hex: bool,
    value_hex: bool,
}

impl<'a> BatchPut<'a> {
    pub fn new(db: DB, matches: &'a ArgMatches<'a>) -> BatchPut<'a> {
        let kv_str: Vec<_> = matches.values_of("KEY-VALUE").unwrap().collect();
        BatchPut {
            db,
            key_values: kv_str,
            key_hex: matches.is_present("key_hex") || matches.is_present("hex"),
            value_hex: matches.is_present("value_hex") || matches.is_present("hex"),
        }
    }
}

impl<'a> Command for BatchPut<'a> {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut batch = WriteBatch::default();
        if self.key_values.len() % 2 != 0 {
            return Err(Box::new(SimpleError::new(format!(
                "Keys and values bnumber has to be even, given {}",
                self.key_values.len()
            ))));
        }
        for i in (0..self.key_values.len()).step_by(2) {
            let k = if self.key_hex {
                utils::hex::decode(self.key_values[i])?
            } else {
                self.key_values[i].as_bytes().into()
            };
            let v = if self.value_hex {
                utils::hex::decode(self.key_values[i + 1])?
            } else {
                self.key_values[i + 1].as_bytes().into()
            };
            batch.put(k, v);
        }
        self.db.write(batch)?;
        println!("OK");
        Ok(())
    }

    fn args() -> App<'static, 'static> {
        SubCommand::with_name(Self::name())
            .about("Puts given key value to the DB")
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
                Arg::with_name("KEY-VALUE")
                    .help("Key provided in hex format")
                    .required(true)
                    .min_values(2),
            )
            .into()
    }

    fn name() -> &'static str {
        "batchput"
    }
}
