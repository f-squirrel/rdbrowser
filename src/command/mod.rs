use clap::ArgMatches;
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
    match matches.subcommand() {
        ("put", Some(_)) => put::Put::new(matches),
        ("get", Some(_)) => get::Get::new(matches),
        ("delete", Some(_)) => delete::Delete::new(matches),
        ("batchput", Some(_)) => batchput::BatchPut::new(matches),
        ("scan", Some(_)) => scan::Scan::new(matches),
        ("deleterange", Some(_)) => deleterange::DeleteRange::new(matches),
        _ => unreachable!(),
    }
}
