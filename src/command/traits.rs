use clap::{App, ArgMatches};
use rocksdb::{Options, DB};
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
    //
    //    fn open_db(opts: &Options) -> Result<DB, rocksdb::Error> {
    //        let db = DB::open_cf(
    //            &opts,
    //            matches.value_of("db").unwrap(),
    //            &[matches.value_of("column_family").unwrap()],
    //        )?;
    //    }

    fn args() -> App<'static, 'static>
    where
        Self: Sized;
    fn name() -> &'static str
    where
        Self: Sized;
    fn run(&self) -> Result<(), Box<dyn std::error::Error>>;
}
