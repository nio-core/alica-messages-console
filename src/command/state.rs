use crate::sawtooth::Client;
use crate::command::{SawtoothCommand, ExecutionResult};
use crate::command::Error::ClientError;

pub struct ListCommand {
    client: Client,
    namespace: String
}

impl ListCommand {
    pub fn new(client: Client, namespace: &str) -> Self {
        ListCommand {
            client,
            namespace: namespace.to_string()
        }
    }
}

impl SawtoothCommand for ListCommand {
    fn execute(&self) -> ExecutionResult {
        let state_entries = self.client.list_state_entries().map_err(|_| ClientError)?;

        println!("Got {} state entries", state_entries.len());

        for entry in state_entries {
            if entry.get_address().starts_with(&self.namespace) {
                println!("ADDRESS: {}", entry.get_address());
                println!("DATA: {:?}", entry.get_data());
            }
        }

        Ok(())
    }
}