use protobuf::Message;
use sawtooth_sdk::messages::transaction::{Transaction, TransactionHeader};
use sawtooth_sdk::messages::batch::{Batch, BatchHeader};
use crate::sawtooth::{TransactionFactory, TransactionPayload, Error, BatchFactory, ComponentFactory, TransactionFamily};
use crate::sawtooth::Error::{SerializationError, SigningError, KeyError};
use crate::sawtooth::helper;
use sawtooth_sdk::signing::Signer;

pub struct GeneralPurposeComponentFactory<'a> {
    transaction_family: TransactionFamily,
    signer: Signer<'a>
}

impl<'a> GeneralPurposeComponentFactory<'a> {
    pub fn new(transaction_family: TransactionFamily, signer: Signer<'a>) -> Self {
        GeneralPurposeComponentFactory {
            transaction_family,
            signer
        }
    }
}

impl<'a> TransactionFactory for GeneralPurposeComponentFactory<'a> {
    fn create_transaction_for(&self, message: &TransactionPayload, header: &TransactionHeader)
                              -> Result<Transaction, Error> {
        let header = header.write_to_bytes().map_err(|_| SerializationError("Transaction Header".to_string()))?;
        let header_signature = self.signer.sign(&header).map_err(|_| SigningError("Transaction Header".to_string()))?;

        let mut transaction = Transaction::new();
        transaction.set_header_signature(header_signature);
        transaction.set_header(header);
        transaction.set_payload(message.serialize());

        Ok(transaction)
    }

    fn create_transaction_header_for(&self, message: &TransactionPayload)
                              -> Result<TransactionHeader, Error> {
        let payload_checksum = helper::calculate_checksum(&message.serialize());
        let state_address = self.transaction_family.calculate_state_address_for(&message);
        let public_key = self.signer.get_public_key().map_err(|_| KeyError("Transaction Header".to_string()))?.as_hex();

        let mut transaction_header = TransactionHeader::new();
        transaction_header.set_family_name(self.transaction_family.name());
        transaction_header.set_family_version(self.transaction_family.version());
        transaction_header.set_nonce(helper::random_nonce());
        transaction_header.set_inputs(protobuf::RepeatedField::from_vec(vec![state_address.clone()]));
        transaction_header.set_outputs(protobuf::RepeatedField::from_vec(vec![state_address]));
        transaction_header.set_signer_public_key(public_key.clone());
        transaction_header.set_batcher_public_key(public_key);
        transaction_header.set_payload_sha512(payload_checksum);

        Ok(transaction_header)
    }
}

impl<'a> BatchFactory for GeneralPurposeComponentFactory<'a> {
    fn create_batch_for(&self, transactions: &Vec<Transaction>, header: &BatchHeader) -> Result<Batch, Error> {
        let header = header.write_to_bytes().map_err(|_| SerializationError("Batch Header".to_string()))?;
        let header_signature = self.signer.sign(&header).map_err(|_| SigningError("Batch Header".to_string()))?;

        let mut batch = Batch::new();
        batch.set_header_signature(header_signature);
        batch.set_header(header);
        batch.set_transactions(protobuf::RepeatedField::from_vec(transactions.to_vec()));

        Ok(batch)
    }

    fn create_batch_header_for(&self, transactions: &Vec<Transaction>) -> Result<BatchHeader, Error> {
        let public_key = self.signer.get_public_key().map_err(|_| KeyError("Batch Header".to_string()))?.as_hex();

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

impl<'a> ComponentFactory for GeneralPurposeComponentFactory<'a> {}
