use sawtooth_sdk::messages::validator::Message_MessageType::{CLIENT_TRANSACTION_LIST_REQUEST,
                                                             CLIENT_TRANSACTION_LIST_RESPONSE,
                                                             CLIENT_BATCH_SUBMIT_REQUEST,
                                                             CLIENT_BATCH_SUBMIT_RESPONSE};
use sawtooth_sdk::messages::client_batch_submit::{ClientBatchSubmitRequest,
                                                  ClientBatchSubmitResponse};
use sawtooth_sdk::messages::client_transaction::{ClientTransactionListRequest,
                                                 ClientTransactionListResponse};
use crate::communication::{AlicaMessage, Client};
use std::borrow::Borrow;

pub trait SawtoothCommand {
    fn execute(&self);
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
    fn execute(&self) {
        let transaction = self.client.transaction_for(self.message.borrow());
        let transactions = vec![transaction];
        let batch = self.client.batch_for(&transactions);

        let mut batch_submit_request = ClientBatchSubmitRequest::new();
        batch_submit_request.set_batches(protobuf::RepeatedField::from_vec(vec![batch]));

        let response = match self.client.send(&batch_submit_request, CLIENT_BATCH_SUBMIT_REQUEST) {
            Ok(message) => message,
            Err(e) => panic!(e)
        };

        let response: ClientBatchSubmitResponse = match response.get_message_type() {
            CLIENT_BATCH_SUBMIT_RESPONSE => {
                protobuf::parse_from_bytes::<ClientBatchSubmitResponse>(response.get_content())
                    .unwrap()
            },
            _ => panic!("Wrong response")
        };

        println!("Batch status: {:?}", response.get_status());
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
    fn execute(&self) {
        let request = ClientTransactionListRequest::new();
        let response = match self.client.send(&request,CLIENT_TRANSACTION_LIST_REQUEST) {
            Ok(message) => message,
            Err(e) => panic!(e)
        };

        let response: ClientTransactionListResponse = match response.get_message_type() {
            CLIENT_TRANSACTION_LIST_RESPONSE => {
                protobuf::parse_from_bytes::<ClientTransactionListResponse>(response.get_content())
                    .unwrap()
            },
            _ => panic!("Wrong response")
        };

        let transactions = response.get_transactions();
        println!("Got {} transactions", transactions.len());

        for transaction in transactions {
            println!("{}", transaction.get_header_signature())
        }
    }
}
