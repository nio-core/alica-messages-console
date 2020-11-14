use crate::sawtooth::{AlicaMessage, Client};
use crate::command::{SawtoothCommand, ExecutionResult};
use crate::command::Error::{ClientError};

pub struct SubmissionCommand<'a> {
    client: &'a Client<'a>,
    message: AlicaMessage
}

impl<'a> SubmissionCommand<'a> {
    pub fn new(client: &'a Client, message: AlicaMessage) -> Self {
        SubmissionCommand {
            client,
            message
        }
    }
}

impl<'a> SawtoothCommand for SubmissionCommand<'a> {
    fn execute(&self) -> ExecutionResult {
        let messages = vec![&self.message];
        self.client.create_batch(&messages).map_err(|_| ClientError)
    }
}

pub struct ListCommand<'a> {
    client: &'a Client<'a>
}

impl<'a> ListCommand<'a> {
    pub fn new(client: &'a Client) -> Self {
        ListCommand {
            client
        }
    }
}

impl<'a> SawtoothCommand for ListCommand<'a> {
    fn execute(&self) -> ExecutionResult {
        let transactions = self.client.list_transactions().map_err(|_| ClientError)?;

        println!("Got {} transactions", transactions.len());
        for transaction in transactions {
            println!("{}", transaction.get_header_signature())
        }

        Ok(())
    }
}