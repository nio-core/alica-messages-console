use sawtooth_sdk::messaging::stream::{MessageConnection, MessageSender};
use sawtooth_sdk::messaging::zmq_stream::ZmqMessageConnection;
use sawtooth_sdk::messages::transaction::Transaction;
use sawtooth_sdk::messages::validator::{self, Message_MessageType};
use sawtooth_sdk::messages::client_state::{ClientStateListRequest, ClientStateListResponse, ClientStateListResponse_Entry};
use sawtooth_sdk::messages::client_batch_submit::{ClientBatchSubmitRequest, ClientBatchSubmitResponse};
use sawtooth_sdk::messages::client_transaction::{ClientTransactionListRequest, ClientTransactionListResponse};
use protobuf::ProtobufEnum;
use crate::sawtooth::Error::{SerializationError, WrongResponse, DeserializationError, RequestError, ResponseError};
use crate::sawtooth::{Error, AlicaMessagePayload, ComponentFactory};

pub struct Client {
    factory: Box<dyn ComponentFactory>,
    connection: ZmqMessageConnection
}

impl Client {
    pub fn new(url: &str, component_factory: Box<dyn ComponentFactory>) -> Self {
        Client {
            factory: component_factory,
            connection: ZmqMessageConnection::new(url)
        }
    }

    pub fn list_state_entries(&self) -> Result<Vec<ClientStateListResponse_Entry>, Error> {
        let request = ClientStateListRequest::new();
        let response = self.send(&request, Message_MessageType::CLIENT_STATE_LIST_REQUEST)?;
        self.validate_response(&response, Message_MessageType::CLIENT_STATE_LIST_RESPONSE)?;
        let response_data = self.parse_response::<ClientStateListResponse>(response)?;
        Ok(response_data.get_entries().to_vec())
    }

    pub fn create_batch(&self, contents: &[&AlicaMessagePayload]) -> Result<(), Error> {
        let mut transactions = Vec::new();
        transactions.reserve(contents.len());
        for message in contents {
            let transaction_header = self.factory.create_transaction_header_for(message)?;
            let transaction = self.factory.create_transaction_for(message, &transaction_header)?;
            transactions.push(transaction);
        }
        let batch_header = self.factory.create_batch_header_for(&transactions)?;
        let batch = self.factory.create_batch_for(&transactions, &batch_header)?;

        let mut batch_submit_request = ClientBatchSubmitRequest::new();
        batch_submit_request.set_batches(protobuf::RepeatedField::from_vec(vec![batch]));

        let response = self.send(&batch_submit_request, Message_MessageType::CLIENT_BATCH_SUBMIT_REQUEST)?;
        self.validate_response(&response, Message_MessageType::CLIENT_BATCH_SUBMIT_RESPONSE)?;
        self.parse_response::<ClientBatchSubmitResponse>(response)?;

        Ok(())
    }

    pub fn list_transactions(&self) -> Result<Vec<Transaction>, Error> {
        let request = ClientTransactionListRequest::new();
        let response = self.send(&request, Message_MessageType::CLIENT_TRANSACTION_LIST_REQUEST)?;
        self.validate_response(&response, Message_MessageType::CLIENT_TRANSACTION_LIST_RESPONSE)?;
        let response_data = self.parse_response::<ClientTransactionListResponse>(response)?;
        Ok(response_data.get_transactions().to_vec())
    }

    pub fn send(&self, request: &dyn protobuf::Message, request_type: Message_MessageType)
                -> Result<validator::Message, Error> {
        let (sender, _) = self.connection.create();
        let correlation_id = uuid::Uuid::new_v4().to_string();
        let message_bytes = &request.write_to_bytes().map_err(|_| SerializationError("Request".to_string()))?;

        sender.send(request_type, &correlation_id, message_bytes)
            .map(|mut future| future.get())
            .map_err(|_| ResponseError)?
            .map_err(|_| RequestError)
    }

    fn validate_response(&self, response: &validator::Message, expected_type: Message_MessageType) -> Result<(), Error> {
        let message_type = response.get_message_type();
        if message_type == expected_type {
            Ok(())
        } else {
            let expected_response_type = expected_type.descriptor().name().to_string();
            let actual_response_type = message_type.descriptor().name().to_string();
            Err(WrongResponse(expected_response_type, actual_response_type))
        }
    }

    fn parse_response<T>(&self, response: validator::Message) -> Result<T, Error>
        where T: protobuf::Message {
        protobuf::parse_from_bytes::<T>(response.get_content()).map_err(|_| DeserializationError)
    }
}