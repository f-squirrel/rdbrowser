use clap::App;

pub trait Command {
    fn args() -> App<'static, 'static>
    where
        Self: Sized;
    fn name() -> &'static str
    where
        Self: Sized;
    fn run(&self);
}
