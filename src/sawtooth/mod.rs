pub mod communication;
pub mod factory;
pub mod helper;

pub use communication::Client;

use sawtooth_sdk::messages::transaction::{Transaction, TransactionHeader};
use sawtooth_sdk::messages::batch::{Batch, BatchHeader};

pub trait ComponentFactory: TransactionFactory + BatchFactory {}

pub trait TransactionFactory {
    fn create_transaction_for(&self, message: &TransactionPayload, header: &TransactionHeader) -> Result<Transaction, Error>;

    fn create_transaction_header_for(&self, message: &TransactionPayload) -> Result<TransactionHeader, Error>;
}

pub trait BatchFactory {
    fn create_batch_for(&self, transactions: &Vec<Transaction>, header: &BatchHeader) -> Result<Batch, Error>;

    fn create_batch_header_for(&self, transactions: &Vec<Transaction>) -> Result<BatchHeader, Error>;
}

pub enum Error {
    RequestError,
    ResponseError,
    WrongResponse(String, String),
    SerializationError(String),
    DeserializationError,
    SigningError(String),
    KeyError(String)
}

pub struct TransactionFamily {
    name: String,
    version: String
}

impl TransactionFamily {
    pub fn new(name: &str, version: &str) -> Self {
        TransactionFamily {
            name: name.to_string(),
            version: version.to_string()
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn version(&self) -> String {
        self.version.clone()
    }

    fn calculate_state_address_for(&self, message: &TransactionPayload) -> String {
        let payload_part = helper::calculate_checksum(
            &format!("{}{}{}", &message.agent_id, &message.message_type, &message.timestamp));
        let namespace_part = helper::calculate_checksum(&self.name);
        format!("{}{}", &namespace_part[..6], &payload_part[..64])
    }
}

#[derive(Debug)]
pub struct TransactionPayload {
    agent_id: String,
    message_type: String,
    message: Vec<u8>,
    timestamp: u64,
}

impl TransactionPayload {
    pub fn new(agent_id: &str, message_type: &str, message: &[u8], timestamp: u64) -> TransactionPayload {
        TransactionPayload {
            agent_id: agent_id.to_string(),
            message_type: message_type.to_string(),
            message: message.to_vec(),
            timestamp,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let message = String::from_utf8(self.message.clone()).expect("Serialization of message failed");
        format!("{}|{}|{}|{}", &self.agent_id, &self.message_type, message, &self.timestamp).as_bytes().to_vec()
    }
}
