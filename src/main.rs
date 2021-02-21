extern crate clap;

mod cmd_parser;
mod commands;

fn main() {
    let cmd = commands::create(cmd_parser::get_cmd_args());
    cmd.run();
}
