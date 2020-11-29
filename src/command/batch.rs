use crate::sawtooth::Client;
use crate::command::{self, SawtoothCommand, ExecutionResult};
use sawtooth_alica_payload::payloads::TransactionPayload;

pub struct CreateCommand<'a> {
    client: Client<'a>,
    message: TransactionPayload
}

impl<'a> CreateCommand<'a> {
    pub fn new(client: Client<'a>, message: TransactionPayload) -> Self {
        CreateCommand {
            client,
            message
        }
    }
}

impl<'a> SawtoothCommand for CreateCommand<'a> {
    fn execute(&self) -> ExecutionResult {
        let messages = vec![&self.message];
        self.client.create_batch(&messages).map_err(|error| command::Error::from(error))
    }
}