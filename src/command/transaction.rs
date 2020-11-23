use crate::sawtooth::Client;
use crate::command::{self, SawtoothCommand, ExecutionResult};
use crate::command::Error::ExecutionError;

pub struct ListCommand<'a> {
    client: Client<'a>
}

impl<'a> ListCommand<'a> {
    pub fn new(client: Client<'a>) -> Self {
        ListCommand {
            client
        }
    }
}

impl<'a> SawtoothCommand for ListCommand<'a> {
    fn execute(&self) -> ExecutionResult {
        let transactions = self.client.list_transactions().map_err(|error| command::Error::from(error))?;

        println!("Got {} transactions", transactions.len());
        for transaction in transactions {
            println!("Transaction ID: {}", transaction.get_header_signature());
            let payload_string = String::from_utf8(transaction.get_payload().to_vec())
                .map_err(|_| ExecutionError("Could no parse payload as UTF8 String".to_string()))?;
            println!("-> Payload: {}", payload_string)
        }

        Ok(())
    }
}