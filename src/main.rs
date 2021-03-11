mod cmd_parser;
mod command;
mod utils;
use command::create;
use std::error::Error;

// We need another function to make sure that all the objects
// are gracefully dropped before std::process::exit is called
fn real_main() -> Result<(), Box<dyn Error>> {
    create(&cmd_parser::build_cmd_args())?.run()
}

fn main() {
    match real_main() {
        Ok(()) => {}
        Err(error) => {
            eprintln!("Failed: {}", error);
            std::process::exit(1);
        }
    }
}
