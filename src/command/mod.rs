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
        ("put", Some(_)) => put::Put::create(matches),
        ("get", Some(_)) => get::Get::create(matches),
        ("delete", Some(_)) => delete::Delete::create(matches),
        ("batchput", Some(_)) => batchput::BatchPut::create(matches),
        ("scan", Some(_)) => scan::Scan::create(matches),
        ("deleterange", Some(_)) => deleterange::DeleteRange::create(matches),
        _ => unreachable!(),
    }
}
