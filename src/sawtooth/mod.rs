pub mod communication;
pub mod factory;

pub use communication::Client;

pub enum Error {
    RequestError,
    ResponseError,
    WrongResponse,
    SerializationError(String),
    DeserializationError,
    SigningError(String),
    KeyError(String)
}

#[derive(Debug)]
pub struct AlicaMessage {
    agent_id: String,
    message_type: String,
    message: String,
    timestamp: String,
}

impl AlicaMessage {
    pub fn new(agent_id: String, message_type: String, message: String, timestamp: String) -> AlicaMessage {
        AlicaMessage {
            agent_id,
            message_type,
            message,
            timestamp,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        format!(
            "{}|{}|{}|{}",
            &self.agent_id, &self.message_type, &self.message, &self.timestamp
        )
            .as_bytes()
            .to_vec()
    }
}


use sawtooth_sdk::messages::transaction::{Transaction, TransactionHeader};
use sawtooth_sdk::messages::batch::{Batch, BatchHeader};
use sawtooth_sdk::signing::Signer;

pub trait ComponentFactory: TransactionFactory + BatchFactory {}

pub trait TransactionFactory {
    fn create_transaction_for(&self, message: &AlicaMessage, header: &TransactionHeader, signer: &Signer) -> Result<Transaction, Error>;

    fn create_transaction_header_for(&self, message: &AlicaMessage, signer: &Signer) -> Result<TransactionHeader, Error>;
}

pub trait BatchFactory {
    fn create_batch_for(&self, transactions: &Vec<Transaction>, header: &BatchHeader, signer: &Signer) -> Result<Batch, Error>;

    fn create_batch_header_for(&self, transactions: &Vec<Transaction>, signer: &Signer) -> Result<BatchHeader, Error>;
}