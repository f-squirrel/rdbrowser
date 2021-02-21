use clap::ArgMatches;
use rocksdb::{Options, DB};
use std::boxed::Box;

mod delete;
mod get;
mod put;
pub mod traits;

pub fn create(matches: ArgMatches) -> Box<dyn traits::Command> {
    let mut opts = Options::default();
    opts.create_if_missing(matches.is_present("create_if_missing"));
    //TODO(DD): handle in a more elegant way, i.e. not crash
    let db = DB::open(&opts, matches.value_of("db").unwrap()).unwrap();
    match matches.subcommand() {
        ("put", Some(put)) => Box::new(put::Put::new(db, put)),
        ("get", Some(get)) => Box::new(get::Get::new(db, get)),
        ("delete", Some(delete)) => Box::new(delete::Delete::new(db, delete)),
        _ => unreachable!(),
    }
}
