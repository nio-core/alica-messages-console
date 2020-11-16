use crate::sawtooth::{Client, AlicaMessagePayload};
use crate::command::{self, SawtoothCommand, ExecutionResult};

pub struct CreateCommand {
    client: Client,
    message: AlicaMessagePayload
}

impl CreateCommand {
    pub fn new(client: Client, message: AlicaMessagePayload) -> Self {
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