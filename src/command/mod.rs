pub mod state;
pub mod transaction;

#[derive(Debug)]
pub enum Error {
    ExecutionError
}

pub type ExecutionResult = Result<(), Error>;

pub trait SawtoothCommand {
    fn execute(&self) -> ExecutionResult;
}
