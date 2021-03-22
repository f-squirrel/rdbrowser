use clap::{App, ArgMatches};
use rocksdb::Options;
use std::boxed::Box;

pub trait Command {
    fn build_options(matches: &ArgMatches) -> Options
    where
        Self: Sized,
    {
        let mut opts = Options::default();
        opts.create_if_missing(matches.is_present("create_if_missing"));
        opts
    }

    fn args() -> App<'static, 'static>
    where
        Self: Sized;
    fn name() -> &'static str
    where
        Self: Sized;
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
