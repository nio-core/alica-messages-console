use crate::sawtooth::Client;
use crate::command::{self, SawtoothCommand, ExecutionResult};
use sawtooth_alica_message_transaction_payload::payloads;
use sawtooth_alica_message_transaction_payload::payloads::TransactionPayload;

pub struct ListCommand<'a> {
    client: Client<'a>,
    payload_format: &'a dyn payloads::Format,
    namespace: String
}

impl<'a> ListCommand<'a> {
    pub fn new(client: Client<'a>, namespace: &str, payload_format: &'a dyn payloads::Format) -> Self {
        ListCommand {
            client,
            payload_format,
            namespace: namespace.to_string()
        }
    }
}

impl<'a> SawtoothCommand for ListCommand<'a> {
    fn execute(&self) -> ExecutionResult {
        let state_entries = self.client.list_state_entries().map_err(|error| command::Error::from(error))?;

        println!("Got {} state entries", state_entries.len());

        let results= state_entries.iter()
            .filter(|entry| entry.get_address().starts_with(&self.namespace))
            .map(|entry| self.payload_format.deserialize(entry.get_data()).map_err(|error| command::Error::from(error)))
            .collect::<Vec<Result<TransactionPayload, command::Error>>>();

        for result in results {
            let transaction = result?;
            println!("Transaction:");
            println!("-> Agent ID: \"{}\"", &transaction.agent_id);
            println!("-> Message Type: \"{}\"", &transaction.message_type);
            println!("-> Message: \"{}\"", String::from_utf8(transaction.message_bytes.clone()).expect("Message is not a string"));
            println!("-> Timestamp of sending: \"{}\"", transaction.timestamp)
        }

        Ok(())
    }
}