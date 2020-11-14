use crate::sawtooth::Client;
use crate::command::{SawtoothCommand, ExecutionResult};
use crate::command::Error::ClientError;

pub struct ListCommand<'a> {
    client: &'a Client<'a>,
    namespace: &'a str
}

impl<'a> ListCommand<'a> {
    pub fn new(client: &'a Client, namespace: &'a str) -> Self {
        ListCommand {
            client,
            namespace
        }
    }
}

impl<'a> SawtoothCommand for ListCommand<'a> {
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