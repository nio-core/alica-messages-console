use crate::sawtooth::Client;
use crate::command::{self, SawtoothCommand, ExecutionResult};

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
        let transactions = self.client.list_transactions().map_err(|error| command::Error::from(error))?;

        println!("Got {} transactions", transactions.len());
        for transaction in transactions {
            println!("{}", transaction.get_header_signature())
        }

        Ok(())
    }
}