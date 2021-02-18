extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("rdBrowser")
        .version("0.1")
        .author("Dmitry Danilov")
        .about("CLI Browser for RocksDB\nhttps://github.com/f-squirrel/rdbrowser")
        .arg(
            Arg::with_name("db")
                .long("db")
                .value_name("FILE")
                .help("Sets path to the target database")
                .required(true)
                .takes_value(true),
        )
        .get_matches();
    println!("Using input file: {}", matches.value_of("db").unwrap());
}
