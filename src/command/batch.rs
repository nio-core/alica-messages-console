use crate::sawtooth::Client;
use crate::command::{self, SawtoothCommand, ExecutionResult};
use sawtooth_alica_message_transaction_payload::payloads::TransactionPayload;

pub struct CreateCommand {
    client: Client,
    message: TransactionPayload
}

impl CreateCommand {
    pub fn new(client: Client, message: TransactionPayload) -> Self {
        CreateCommand {
            client,
            message
        }
    }
}

impl SawtoothCommand for CreateCommand {
    fn execute(&self) -> ExecutionResult {
        let messages = vec![&self.message];
        self.client.create_batch(&messages).map_err(|error| command::Error::from(error))
    }
}