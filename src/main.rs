mod cmd_parser;
mod command;
mod utils;
use command::create;
extern crate simple_error;

fn main() {
    let args = cmd_parser::build_cmd_args();
    let cmd = create(&args);
    if let Err(_error) = cmd.run() {
        eprintln!("Failed: {}", _error);
        drop(cmd);
        std::process::exit(1);
    }
}
