pub mod communication;

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
    pub fn new(
        agent_id: String,
        message_type: String,
        message: String,
        timestamp: String,
    ) -> AlicaMessage {
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
