pub mod batch;
pub mod state;

use crate::sawtooth;
use sawtooth_alica_payload::payloads;

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
            sawtooth::Error::KeyError(component) => format!("Failed to fetch public key for {}", component),
            sawtooth::Error::InvalidBatch => format!("Invalid batch"),
            sawtooth::Error::InternalError => format!("Internal error when submitting batch"),
            sawtooth::Error::FullQueue => format!("Batch request queue of targeted validator is full!"),
            sawtooth::Error::BatchStatusUnset => format!("No status set for batch"),
        };

        Error::ExecutionError(message)
    }
}

impl From<payloads::Error> for Error {
    fn from(error: payloads::Error) -> Self {
        let message = match error {
            payloads::Error::InvalidPayload(message) => message,
            payloads::Error::InvalidTimestamp => "Invalid timestamp supplied".to_string()
        };

        Error::ExecutionError(message)
    }
}

pub type ExecutionResult = Result<(), Error>;

pub trait SawtoothCommand {
    fn execute(&self) -> ExecutionResult;
}
