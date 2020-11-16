use crate::sawtooth::{AlicaMessagePayload, Client};
use crate::command::{SawtoothCommand, ExecutionResult};
use crate::command::Error::{ClientError};

pub struct BatchCreationCommand {
    client: Client,
    message: AlicaMessagePayload
}

impl BatchCreationCommand {
    pub fn new(client: Client, message: AlicaMessagePayload) -> Self {
        BatchCreationCommand {
            client,
            message
        }
    }
}

impl SawtoothCommand for BatchCreationCommand {
    fn execute(&self) -> ExecutionResult {
        let messages = vec![&self.message];
        self.client.create_batch(&messages).map_err(|_| ClientError)
    }
}

pub struct ListCommand {
    client: Client
}

impl ListCommand {
    pub fn new(client: Client) -> Self {
        ListCommand {
            client
        }
    }
}

impl SawtoothCommand for ListCommand {
    fn execute(&self) -> ExecutionResult {
        let transactions = self.client.list_transactions().map_err(|_| ClientError)?;

        println!("Got {} transactions", transactions.len());
        for transaction in transactions {
            println!("{}", transaction.get_header_signature())
        }

        Ok(())
    }
}