use clap::ArgMatches;
use rocksdb::{Options, DB};
use std::boxed::Box;

pub mod batchput;
pub mod delete;
pub mod deleterange;
pub mod get;
pub mod put;
pub mod scan;
pub mod traits;

pub fn create<'a>(matches: &'a ArgMatches<'a>) -> Box<dyn traits::Command + 'a> {
    let mut opts = Options::default();
    opts.create_if_missing(matches.is_present("create_if_missing"));
    //TODO(DD): handle in a more elegant way, i.e. not crash

    let db = DB::open_cf(
        &opts,
        matches.value_of("db").unwrap(),
        &[matches.value_of("column_family").unwrap()],
    )
    .unwrap();
    match matches.subcommand() {
        ("put", Some(put)) => Box::new(put::Put::new(db, put)),
        ("get", Some(get)) => Box::new(get::Get::new(db, get)),
        ("delete", Some(delete)) => Box::new(delete::Delete::new(db, delete)),
        ("batchput", Some(batchput)) => Box::new(batchput::BatchPut::new(db, batchput)),
        ("scan", Some(scan)) => Box::new(scan::Scan::new(db, scan)),
        ("deleterange", Some(deleterange)) => {
            Box::new(deleterange::DeleteRange::new(db, deleterange))
        }
        _ => unreachable!(),
    }
}
