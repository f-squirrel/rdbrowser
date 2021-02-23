use clap::App;
use std::boxed::Box;
use std::error::Error;

pub trait Command {
    fn args() -> App<'static, 'static>
    where
        Self: Sized;
    fn name() -> &'static str
    where
        Self: Sized;
    fn run(&self) -> Result<(), Box<dyn Error>>;
}
