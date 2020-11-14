pub mod state;
pub mod transaction;

#[derive(Debug)]
pub enum CommunicationError {
    ValidatorNotReachable,
    InvalidResponse,
    WrongResponse
}

pub trait SawtoothCommand {
    fn execute(&self) -> Result<(), CommunicationError>;
}
