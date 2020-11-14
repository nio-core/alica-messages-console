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
use crate::sawtooth::Error::{SerializationError, WrongResponse, DeserializationError, RequestError, ResponseError, SigningError, KeyError};
use crate::sawtooth::{Error, AlicaMessage};

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
        let response = self.send(&request, Message_MessageType::CLIENT_STATE_LIST_REQUEST)?;
        self.validate_response(&response, Message_MessageType::CLIENT_STATE_LIST_REQUEST)?;
        let response_data = self.parse_response::<ClientStateListResponse>(response)?;
        Ok(response_data.get_entries().to_vec())
    }

    pub fn create_batch(&self, contents: &[&AlicaMessage]) -> Result<(), Error> {
        let mut transactions = Vec::new();
        transactions.reserve(contents.len());
        for message in contents {
            let transaction = self.transaction_for(message, &self.family_name, &self.family_version, &self.signer)?;
            transactions.push(transaction);
        }
        let batch = self.batch_for(&transactions, &self.signer)?;

        let mut batch_submit_request = ClientBatchSubmitRequest::new();
        batch_submit_request.set_batches(protobuf::RepeatedField::from_vec(vec![batch]));

        let response = self.send(&batch_submit_request, Message_MessageType::CLIENT_BATCH_SUBMIT_REQUEST)?;
        self.validate_response(&response, Message_MessageType::CLIENT_BATCH_SUBMIT_REQUEST)?;
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
            Err(WrongResponse)
        }
    }

    fn parse_response<T>(&self, response: validator::Message) -> Result<T, Error>
        where T: protobuf::Message {
        protobuf::parse_from_bytes::<T>(response.get_content()).map_err(|_| DeserializationError)
    }

    fn transaction_for(&self, message: &AlicaMessage, family_name: &str, family_version: &str, signer: &Signer) -> Result<Transaction, Error> {
        let header = self.transaction_header_for(message, family_name, family_version, signer)
            .write_to_bytes()
            .map_err(|_| SerializationError("Transaction Header".to_string()))?;
        let header_signature = signer.sign(&header).map_err(|_| SigningError("Transaction Header".to_string()))?;

        let mut transaction = Transaction::new();
        transaction.set_header_signature(header_signature);
        transaction.set_header(header);
        transaction.set_payload(message.serialize());

        Ok(transaction)
    }

    fn transaction_header_for(&self, message: &AlicaMessage, family_name: &str, family_version: &str, signer: &Signer) -> TransactionHeader {
        let payload_checksum = helper::calculate_checksum(&message.serialize());
        let state_address = self.state_address_for(&message, family_name);
        let public_key = signer.get_public_key().expect("Error retrieving signer's public key").as_hex();

        let mut transaction_header = TransactionHeader::new();
        transaction_header.set_family_name(family_name.to_string());
        transaction_header.set_family_version(family_version.to_string());
        transaction_header.set_nonce(helper::random_nonce());
        transaction_header.set_inputs(protobuf::RepeatedField::from_vec(vec![state_address.clone()]));
        transaction_header.set_outputs(protobuf::RepeatedField::from_vec(vec![state_address]));
        transaction_header.set_signer_public_key(public_key.clone());
        transaction_header.set_batcher_public_key(public_key);
        transaction_header.set_payload_sha512(payload_checksum);

        transaction_header
    }

    fn batch_for(&self, transactions: &Vec<Transaction>, signer: &Signer) -> Result<Batch, Error> {
        let header = self.batch_header_for(transactions, signer)?.write_to_bytes()
            .map_err(|_| SerializationError("Batch Header".to_string()))?;
        let header_signature = signer.sign(&header).map_err(|_| SigningError("Batch Header".to_string()))?;

        let mut batch = Batch::new();
        batch.set_header_signature(header_signature);
        batch.set_header(header);
        batch.set_transactions(protobuf::RepeatedField::from_vec(transactions.to_vec()));

        Ok(batch)
    }

    fn batch_header_for(&self, transactions: &Vec<Transaction>, signer: &Signer) -> Result<BatchHeader, Error> {
        let public_key = signer.get_public_key().map_err(|_| KeyError("Batch Header".to_string()))?.as_hex();

        let mut header = BatchHeader::new();
        header.set_signer_public_key(public_key);
        header.set_transaction_ids(protobuf::RepeatedField::from_vec(
            transactions
                .iter()
                .map(|transaction| String::from(transaction.get_header_signature()))
                .collect(),
        ));

        Ok(header)
    }

    fn state_address_for(&self, message: &AlicaMessage, family_name: &str) -> String {
        let payload_part = helper::calculate_checksum(
            &format!("{}{}{}", &message.agent_id, &message.message_type, &message.timestamp));

        let namespace_part = helper::calculate_checksum(&family_name);

        format!("{}{}", &namespace_part[..6], &payload_part[..64])
    }

    pub fn get_namespace(&self) -> String {
        let namespace = helper::calculate_checksum(&self.family_name);
        let prefix = &namespace[..6];
        String::from(prefix)
    }
}