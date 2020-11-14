pub mod state;
pub mod transaction;

#[derive(Debug)]
pub enum Error {
    ClientError,
    ExecutionError
}

pub type ExecutionResult = Result<(), Error>;

pub trait SawtoothCommand {
    fn execute(&self) -> ExecutionResult;
}
