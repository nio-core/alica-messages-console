use crate::sawtooth::Client;
use crate::command::{SawtoothCommand, ExecutionResult};
use crate::command::Error::{ClientError};

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