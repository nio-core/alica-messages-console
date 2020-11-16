pub mod batch;
pub mod state;
pub mod transaction;

use crate::sawtooth;

#[derive(Debug)]
pub enum Error {
    ExecutionError(String)
}

impl From<sawtooth::Error> for Error {
    fn from(error: sawtooth::Error) -> Self {
        let message = match error {
            sawtooth::Error::RequestError => "Failed to send request".to_string(),
            sawtooth::Error::ResponseError => "Failed to evaluate response".to_string(),
            sawtooth::Error::WrongResponse(expected_response_type, actual_response_type) =>
                format!("Got wrong response, expected {} but was {}", expected_response_type, actual_response_type),
            sawtooth::Error::SerializationError(component) => format!("Failed to serialize {}", component),
            sawtooth::Error::DeserializationError => "Failed to deserialize response".to_string(),
            sawtooth::Error::SigningError(component) => format!("Failed to sign {}", component),
            sawtooth::Error::KeyError(component) => format!("Failed to fetch public key for {}", component)
        };

        Error::ExecutionError(message)
    }
}

pub type ExecutionResult = Result<(), Error>;

pub trait SawtoothCommand {
    fn execute(&self) -> ExecutionResult;
}
