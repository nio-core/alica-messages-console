use crate::sawtooth::Client;
use crate::command::{self, SawtoothCommand, ExecutionResult};

pub struct ListCommand<'a> {
    client: Client<'a>,
    namespace: String
}

impl<'a> ListCommand<'a> {
    pub fn new(client: Client<'a>, namespace: &str) -> Self {
        ListCommand {
            client,
            namespace: namespace.to_string()
        }
    }
}

impl<'a> SawtoothCommand for ListCommand<'a> {
    fn execute(&self) -> ExecutionResult {
        let state_entries = self.client.list_state_entries().map_err(|error| command::Error::from(error))?;

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