use sawtooth_sdk::messages::validator::Message_MessageType::{CLIENT_TRANSACTION_LIST_REQUEST,
                                                             CLIENT_TRANSACTION_LIST_RESPONSE,
                                                             CLIENT_BATCH_SUBMIT_REQUEST,
                                                             CLIENT_BATCH_SUBMIT_RESPONSE};
use sawtooth_sdk::messages::client_batch_submit::{ClientBatchSubmitRequest, ClientBatchSubmitResponse};
use sawtooth_sdk::messages::client_transaction::{ClientTransactionListRequest, ClientTransactionListResponse};
use crate::communication::{Client, AlicaMessage};
use crate::command::{SawtoothCommand};
use crate::command::CommunicationError::{self, ValidatorNotReachable, InvalidResponse, WrongResponse};

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