use sawtooth_sdk::messages::validator::Message_MessageType::{CLIENT_STATE_LIST_REQUEST,
                                                             CLIENT_STATE_LIST_RESPONSE};
use sawtooth_sdk::messages::client_state::{ClientStateListRequest, ClientStateListResponse};
use crate::communication::Client;
use crate::command::{SawtoothCommand, CommunicationError};
use crate::command::CommunicationError::{ValidatorNotReachable, InvalidResponse, WrongResponse};

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
    fn execute(&self) -> Result<(), CommunicationError> {
        let request = ClientStateListRequest::new();
        let response = self.client.send(&request,CLIENT_STATE_LIST_REQUEST)
            .map_err(|_| ValidatorNotReachable)?;

        let response: ClientStateListResponse = match response.get_message_type() {
            CLIENT_STATE_LIST_RESPONSE => protobuf::parse_from_bytes(response.get_content())
                .map_err(|_| InvalidResponse),
            _ => Err(WrongResponse)
        }?;

        let state = response.get_entries();
        println!("Got {} state entries", state.len());

        for entry in state {
            if entry.get_address().starts_with(&self.client.get_namespace()) {
                println!("ADDRESS: {}", entry.get_address());
                println!("DATA: {:?}", entry.get_data());
            }
        }

        Ok(())
    }
}