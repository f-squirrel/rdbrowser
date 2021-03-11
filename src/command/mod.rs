use clap::ArgMatches;
use rocksdb::{Options, DB};
use std::boxed::Box;
use std::error::Error;

pub mod batchput;
pub mod delete;
pub mod deleterange;
pub mod get;
pub mod put;
pub mod scan;
pub mod traits;

pub fn create<'a>(
    matches: &'a ArgMatches<'a>,
) -> Result<Box<dyn traits::Command + 'a>, Box<dyn Error>> {
    let mut opts = Options::default();
    opts.create_if_missing(matches.is_present("create_if_missing"));
    //TODO(DD): handle in a more elegant way, i.e. not crash

    let db = DB::open_cf(
        &opts,
        matches.value_of("db").unwrap(),
        &[matches.value_of("column_family").unwrap()],
    )?;
    match matches.subcommand() {
        ("put", Some(put)) => Ok(Box::new(put::Put::new(db, put))),
        ("get", Some(get)) => Ok(Box::new(get::Get::new(db, get))),
        ("delete", Some(delete)) => Ok(Box::new(delete::Delete::new(db, delete))),
        ("batchput", Some(batchput)) => Ok(Box::new(batchput::BatchPut::new(db, batchput))),
        ("scan", Some(scan)) => Ok(Box::new(scan::Scan::new(db, scan))),
        ("deleterange", Some(deleterange)) => {
            Ok(Box::new(deleterange::DeleteRange::new(db, deleterange)))
        }
        _ => unreachable!(),
    }
}
