use protobuf::Message;
use sawtooth_sdk::messaging::stream::{MessageConnection, MessageSender};
use sawtooth_sdk::messages::transaction::{Transaction, TransactionHeader};
use sawtooth_sdk::messages::batch::{Batch, BatchHeader};
use sawtooth_sdk::messages::validator::{self, Message_MessageType};
use sawtooth_sdk::messages::client_state::{ClientStateListRequest, ClientStateListResponse, ClientStateListResponse_Entry};
use sawtooth_sdk::messages::client_batch_submit::{ClientBatchSubmitRequest, ClientBatchSubmitResponse};
use sawtooth_sdk::messages::client_transaction::{ClientTransactionListRequest, ClientTransactionListResponse};
use sawtooth_sdk::signing::Signer;
use sawtooth_sdk::messaging::zmq_stream::ZmqMessageConnection;
use crate::helper;
use crate::communication::Error::ClientError;

pub enum Error {
    ClientError
}

pub struct Client<'a> {
    family_name: String,
    family_version: String,
    signer: Signer<'a>,
    connection: ZmqMessageConnection
}

impl<'a> Client<'a> {
    pub fn new(url: &str) -> Self {
        let context = sawtooth_sdk::signing::create_context("secp256k1")
            .expect("Invalid algorithm name in context creation");
        let private_key = context.new_random_private_key()
            .expect("Error creating a private key");

        Client {
            family_name: String::from("alica_messages"),
            family_version: String::from("0.1.0"),
            signer: Signer::new_boxed(context, private_key),
            connection: ZmqMessageConnection::new(url)
        }
    }

    pub fn list_state_entries(&self) -> Result<Vec<ClientStateListResponse_Entry>, Error> {
        let request = ClientStateListRequest::new();
        let response = self.send(&request, Message_MessageType::CLIENT_STATE_LIST_REQUEST)
            .map_err(|_| ClientError)?;
        let response = self.parse_response::<ClientStateListResponse>(response, Message_MessageType::CLIENT_STATE_LIST_RESPONSE)?;
        Ok(response.get_entries().to_vec())
    }

    pub fn create_batch(&self, contents: &[&AlicaMessage]) -> Result<(), Error> {
        let transactions = contents.iter()
            .map(|message| self.transaction_for(message))
            .collect();
        let batch = self.batch_for(&transactions);

        let mut batch_submit_request = ClientBatchSubmitRequest::new();
        batch_submit_request.set_batches(protobuf::RepeatedField::from_vec(vec![batch]));

        let response = self.send(&batch_submit_request, Message_MessageType::CLIENT_BATCH_SUBMIT_REQUEST)
            .map_err(|_| ClientError)?;
        let _response = self.parse_response::<ClientBatchSubmitResponse>(response, Message_MessageType::CLIENT_BATCH_SUBMIT_RESPONSE)?;

        Ok(())
    }

    pub fn list_transactions(&self) -> Result<Vec<Transaction>, Error> {
        let request = ClientTransactionListRequest::new();
        let response = self.send(&request, Message_MessageType::CLIENT_TRANSACTION_LIST_REQUEST)
            .map_err(|_| ClientError)?;

        let response = self.parse_response::<ClientTransactionListResponse>(response,Message_MessageType::CLIENT_TRANSACTION_LIST_RESPONSE)?;
        Ok(response.get_transactions().to_vec())
    }

    pub fn send(&self, request: &dyn protobuf::Message, request_type: Message_MessageType)
                -> Result<validator::Message, Error> {
        let (sender, _receiver) = self.connection.create();
        let correlation_id = uuid::Uuid::new_v4().to_string();
        let message_bytes = &request.write_to_bytes().unwrap();

        match sender.send(request_type, correlation_id.as_str(), message_bytes) {
            Ok(mut future) => match future.get() {
                Ok(message) => Ok(message),
                Err(_) => Err(ClientError)
            },
            Err(_) => Err(ClientError)
        }
    }

    fn parse_response<T>(&self, response: validator::Message, expected_response_type: Message_MessageType) -> Result<T, Error>
        where T: protobuf::Message {
        let message_type = response.get_message_type();
        if message_type == expected_response_type {
            protobuf::parse_from_bytes::<T>(response.get_content()).map_err(|_| ClientError)
        } else {
            Err(ClientError)
        }
    }

    fn transaction_for(&self, message: &AlicaMessage) -> Transaction {
        let header = self.transaction_header_for(message).write_to_bytes()
            .expect("Error serializing transaction header");

        let mut transaction = Transaction::new();
        transaction.set_header_signature(
            self.signer.sign(&header)
                .expect("Error signing transaction header"),
        );
        transaction.set_header(header);
        transaction.set_payload(message.serialize());

        transaction
    }

    fn transaction_header_for(&self, message: &AlicaMessage) -> TransactionHeader {
        let payload_checksum = helper::calculate_checksum(&message.serialize());
        let state_address = self.state_address_for(&message);

        let mut transaction_header = TransactionHeader::new();
        transaction_header.set_family_name(self.family_name.clone());
        transaction_header.set_family_version(self.family_version.clone());
        transaction_header.set_nonce(helper::random_nonce());
        transaction_header.set_inputs(protobuf::RepeatedField::from_vec(vec![state_address.clone()]));
        transaction_header.set_outputs(protobuf::RepeatedField::from_vec(vec![state_address]));
        transaction_header.set_signer_public_key(
            self.signer.get_public_key()
                .expect("Error retrieving signer's public key")
                .as_hex(),
        );
        transaction_header.set_batcher_public_key(
            self.signer.get_public_key()
                .expect("Error retrieving signer's public key")
                .as_hex(),
        );
        transaction_header.set_payload_sha512(payload_checksum);

        transaction_header
    }

    fn batch_for(&self, transactions: &Vec<Transaction>) -> Batch {
        let header = self.batch_header_for(transactions).write_to_bytes()
            .expect("Error serializing batch header");

        let mut batch = Batch::new();
        batch.set_header_signature(
            self.signer.sign(&header)
                .expect("Error signing batch header"),
        );
        batch.set_header(header);
        batch.set_transactions(protobuf::RepeatedField::from_vec(transactions.to_vec()));

        batch
    }

    fn batch_header_for(&self, transactions: &Vec<Transaction>) -> BatchHeader {
        let mut header = BatchHeader::new();
        header.set_signer_public_key(
            self.signer.get_public_key()
                .expect("Error retrieving signer's public key")
                .as_hex(),
        );
        header.set_transaction_ids(protobuf::RepeatedField::from_vec(
            transactions
                .iter()
                .map(|t| String::from(t.get_header_signature()))
                .collect(),
        ));

        header
    }

    fn state_address_for(&self, message: &AlicaMessage) -> String {
        let payload_part = helper::calculate_checksum(
            &format!("{}{}{}", &message.agent_id, &message.message_type, &message.timestamp));

        let namespace_part = helper::calculate_checksum(&self.family_name);

        format!("{}{}", &namespace_part[..6], &payload_part[..64])
    }

    pub fn get_namespace(&self) -> String {
        let namespace = helper::calculate_checksum(&self.family_name);
        let prefix = &namespace[..6];
        String::from(prefix)
    }
}

#[derive(Debug)]
pub struct AlicaMessage {
    agent_id: String,
    message_type: String,
    message: String,
    timestamp: String,
}

impl AlicaMessage {
    pub fn new(
        agent_id: String,
        message_type: String,
        message: String,
        timestamp: String,
    ) -> AlicaMessage {
        AlicaMessage {
            agent_id,
            message_type,
            message,
            timestamp,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        format!(
            "{}|{}|{}|{}",
            &self.agent_id, &self.message_type, &self.message, &self.timestamp
        )
            .as_bytes()
            .to_vec()
    }
}
