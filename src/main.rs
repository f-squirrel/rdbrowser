mod cmd_parser;
mod command;
use command::create;

fn main() {
    let cmd = create(cmd_parser::get_cmd_args());
    cmd.run();
}
