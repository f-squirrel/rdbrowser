mod cmd_parser;
mod command;
mod utils;
use command::create;

fn main() {
    let cmd = create(cmd_parser::get_cmd_args());
    if let Err(_error) = cmd.run() {
        eprintln!("Failed: {}", _error);
        drop(cmd);
        std::process::exit(1);
    }
}
