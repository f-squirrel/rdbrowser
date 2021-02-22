use clap::App;

pub trait Command {
    fn args() -> App<'static, 'static> where Self: Sized;
    fn run(&self);
}
