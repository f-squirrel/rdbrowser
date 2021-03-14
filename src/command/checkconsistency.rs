use crate::command::traits::Command;
use clap::{App, ArgMatches, SubCommand};
use rocksdb::DB;
use std::boxed::Box;
use std::error::Error;

#[derive(Debug)]
pub struct CheckConsistency {
    db: DB,
}

impl<'a> CheckConsistency {
    pub fn create(matches: &'a ArgMatches<'a>) -> Result<Box<dyn Command + 'a>, Box<dyn Error>> {
        let mut opts = Self::build_options(matches);
        opts.set_paranoid_checks(true);
        opts.set_num_levels(64);
        let db = DB::open_cf(
            &opts,
            matches.value_of("db").unwrap(),
            &[matches.value_of("column_family").unwrap()],
        )?;
        Ok(std::boxed::Box::new(CheckConsistency { db }))
    }
}

impl<'a> Command for CheckConsistency {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("OK");
        Ok(())
    }

    fn args() -> App<'static, 'static> {
        SubCommand::with_name(Self::name()).about("Checks consistency of the DB")
    }

    fn name() -> &'static str {
        "checkconsistency"
    }
}
