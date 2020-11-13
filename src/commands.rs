use sawtooth_sdk::messages::validator::Message_MessageType::{CLIENT_TRANSACTION_LIST_REQUEST,
                                                             CLIENT_TRANSACTION_LIST_RESPONSE,
                                                             CLIENT_BATCH_SUBMIT_REQUEST,
                                                             CLIENT_BATCH_SUBMIT_RESPONSE,
                                                             CLIENT_STATE_LIST_REQUEST,
                                                             CLIENT_STATE_LIST_RESPONSE};
use sawtooth_sdk::messages::client_batch_submit::{ClientBatchSubmitRequest, ClientBatchSubmitResponse};
use sawtooth_sdk::messages::client_transaction::{ClientTransactionListRequest, ClientTransactionListResponse};
use sawtooth_sdk::messages::client_state::{ClientStateListRequest, ClientStateListResponse};
use crate::communication::{AlicaMessage, Client};
use crate::commands::CommunicationError::{ValidatorNotReachable, WrongResponse, InvalidResponse};

#[derive(Debug)]
pub enum CommunicationError {
    ValidatorNotReachable,
    InvalidResponse,
    WrongResponse
}

pub trait SawtoothCommand {
    fn execute(&self) -> Result<(), CommunicationError>;
}

pub struct TransactionSubmissionCommand<'a> {
    client: &'a Client<'a>,
    message: AlicaMessage
}

impl<'a> TransactionSubmissionCommand<'a> {
    pub fn new(client: &'a Client, message: AlicaMessage) -> Self {
        TransactionSubmissionCommand {
            client,
            message
        }
    }
}

impl<'a> SawtoothCommand for TransactionSubmissionCommand<'a> {
    fn execute(&self) -> Result<(), CommunicationError> {
        let transaction = self.client.transaction_for(&self.message);
        let transactions = vec![transaction];
        let batch = self.client.batch_for(&transactions);

        let mut batch_submit_request = ClientBatchSubmitRequest::new();
        batch_submit_request.set_batches(protobuf::RepeatedField::from_vec(vec![batch]));

        let response = self.client.send(&batch_submit_request, CLIENT_BATCH_SUBMIT_REQUEST)
            .map_err(|_| ValidatorNotReachable)?;

        let _response: ClientBatchSubmitResponse = match response.get_message_type() {
            CLIENT_BATCH_SUBMIT_RESPONSE => protobuf::parse_from_bytes(response.get_content())
                    .map_err(|_| InvalidResponse),
            _ => Err(WrongResponse)
        }?;

        Ok(())
    }
}

pub struct TransactionListCommand<'a> {
    client: &'a Client<'a>
}

impl<'a> TransactionListCommand<'a> {
    pub fn new(client: &'a Client) -> Self {
        TransactionListCommand {
            client
        }
    }
}

impl<'a> SawtoothCommand for TransactionListCommand<'a> {
    fn execute(&self) -> Result<(), CommunicationError> {
        let request = ClientTransactionListRequest::new();
        let response = self.client.send(&request,CLIENT_TRANSACTION_LIST_REQUEST)
            .map_err(|_| ValidatorNotReachable)?;

        let response: ClientTransactionListResponse = match response.get_message_type() {
            CLIENT_TRANSACTION_LIST_RESPONSE => protobuf::parse_from_bytes(response.get_content())
                    .map_err(|_| InvalidResponse),
            _ => Err(WrongResponse)
        }?;

        let transactions = response.get_transactions();
        println!("Got {} transactions", transactions.len());

        for transaction in transactions {
            println!("{}", transaction.get_header_signature())
        }

        Ok(())
    }
}

pub struct StateListCommand<'a> {
    client: &'a Client<'a>
}

impl<'a> StateListCommand<'a> {
    pub fn new(client: &'a Client) -> Self {
        StateListCommand {
            client
        }
    }
}

impl<'a> SawtoothCommand for StateListCommand<'a> {
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