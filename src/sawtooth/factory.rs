use protobuf::Message;
use sawtooth_sdk::messages::transaction::{Transaction, TransactionHeader};
use sawtooth_sdk::messages::batch::{Batch, BatchHeader};
use crate::sawtooth::{TransactionFactory, AlicaMessage, Error, BatchFactory, ComponentFactory};
use crate::sawtooth::Error::{SerializationError, SigningError, KeyError};
use crate::helper;
use sawtooth_sdk::signing::Signer;

pub struct AlicaMessageComponentFactory {
    family_name: String,
    family_version: String
}

impl AlicaMessageComponentFactory {
    pub fn new() -> Self {
        AlicaMessageComponentFactory {
            family_name: "alica_messages".to_string(),
            family_version: "0.1.0".to_string()
        }
    }

    fn state_address_for(&self, message: &AlicaMessage, family_name: &str) -> String {
        let payload_part = helper::calculate_checksum(
            &format!("{}{}{}", &message.agent_id, &message.message_type, &message.timestamp));

        let namespace_part = helper::calculate_checksum(&family_name);

        format!("{}{}", &namespace_part[..6], &payload_part[..64])
    }
}

impl TransactionFactory for AlicaMessageComponentFactory {
    fn create_transaction_for(&self, message: &AlicaMessage, header: &TransactionHeader, signer: &Signer)
        -> Result<Transaction, Error> {
        let header = header.write_to_bytes().map_err(|_| SerializationError("Transaction Header".to_string()))?;
        let header_signature = signer.sign(&header).map_err(|_| SigningError("Transaction Header".to_string()))?;

        let mut transaction = Transaction::new();
        transaction.set_header_signature(header_signature);
        transaction.set_header(header);
        transaction.set_payload(message.serialize());

        Ok(transaction)
    }

    fn create_transaction_header_for(&self, message: &AlicaMessage, signer: &Signer)
                              -> Result<TransactionHeader, Error> {
        let payload_checksum = helper::calculate_checksum(&message.serialize());
        let state_address = self.state_address_for(&message, &self.family_name);
        let public_key = signer.get_public_key().map_err(|_| KeyError("Transaction Header".to_string()))?.as_hex();

        let mut transaction_header = TransactionHeader::new();
        transaction_header.set_family_name(self.family_name.clone());
        transaction_header.set_family_version(self.family_version.clone());
        transaction_header.set_nonce(helper::random_nonce());
        transaction_header.set_inputs(protobuf::RepeatedField::from_vec(vec![state_address.clone()]));
        transaction_header.set_outputs(protobuf::RepeatedField::from_vec(vec![state_address]));
        transaction_header.set_signer_public_key(public_key.clone());
        transaction_header.set_batcher_public_key(public_key);
        transaction_header.set_payload_sha512(payload_checksum);

        Ok(transaction_header)
    }
}

impl BatchFactory for AlicaMessageComponentFactory {
    fn create_batch_for(&self, transactions: &Vec<Transaction>, header: &BatchHeader, signer: &Signer) -> Result<Batch, Error> {
        let header = header.write_to_bytes().map_err(|_| SerializationError("Batch Header".to_string()))?;
        let header_signature = signer.sign(&header).map_err(|_| SigningError("Batch Header".to_string()))?;

        let mut batch = Batch::new();
        batch.set_header_signature(header_signature);
        batch.set_header(header);
        batch.set_transactions(protobuf::RepeatedField::from_vec(transactions.to_vec()));

        Ok(batch)
    }

    fn create_batch_header_for(&self, transactions: &Vec<Transaction>, signer: &Signer) -> Result<BatchHeader, Error> {
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
}

impl ComponentFactory for AlicaMessageComponentFactory {}
