use crate::sawtooth::Client;
use crate::command::{self, SawtoothCommand, ExecutionResult};
use sawtooth_alica_message_transaction_payload::payloads;
use sawtooth_alica_message_transaction_payload::payloads::TransactionPayload;
use crate::filter::TransactionPayloadFilter;

pub struct ListCommand<'a> {
    client: Client<'a>,
    payload_format: &'a dyn payloads::Format,
    namespace: String,
    filters: Vec<Box<dyn TransactionPayloadFilter>>
}

impl<'a> ListCommand<'a> {
    pub fn new(client: Client<'a>, namespace: &str, payload_format: &'a dyn payloads::Format, filters: Vec<Box<dyn TransactionPayloadFilter>>) -> Self {
        ListCommand {
            client,
            payload_format,
            namespace: namespace.to_string(),
            filters
        }
    }
}

impl<'a> SawtoothCommand for ListCommand<'a> {
    fn execute(&self) -> ExecutionResult {
        let state_entries = self.client.list_state_entries().map_err(|error| command::Error::from(error))?;

        println!("Got {} state entries", state_entries.len());

        let mut payloads: Vec<TransactionPayload> = state_entries.iter()
            .filter(|entry| entry.get_address().starts_with(&self.namespace))
            .map(|entry| self.payload_format.deserialize(entry.get_data()).map_err(|error| command::Error::from(error)))
            .collect::<Result<Vec<TransactionPayload>, command::Error>>()?;

        for filter in &self.filters {
            filter.filter(&mut payloads);
        }

        for payload in payloads {
            let transaction = payload;
            println!("Transaction:");
            println!("-> Agent ID: \"{}\"", &transaction.agent_id);
            println!("-> Message Type: \"{}\"", &transaction.message_type);
            println!("-> Message: \"{}\"", String::from_utf8(transaction.message_bytes.clone()).expect("Message is not a string"));
            println!("-> Timestamp of sending: \"{}\"", transaction.timestamp)
        }

        Ok(())
    }
}