pub struct Client<'a> {
    validator_url: &'a str,
}

impl<'a> Client<'a> {
    pub fn new(url: &'a str) -> Self {
        Client { validator_url: url }
    }

    pub fn send(&self, message: &'a str) {
        let context = sawtooth_sdk::signing::create_context("secp256k1")
            .expect("Invalid algorithm name in context cration");
        let private_key = context
            .new_random_private_key()
            .expect("error creating a private key");
        let crypto_factory = sawtooth_sdk::signing::CryptoFactory::new(context.as_ref());
        let signer = crypto_factory.new_signer(private_key.as_ref());

        let payload = Payload::new(String::from("msg"), String::from(message));
        let payload_bytes = payload.serialize();

        let mut nonce = [0u8, 16];
        use rand::Rng;
        rand::thread_rng()
            .try_fill(&mut nonce)
            .expect("Error filling nonce");

        use sha2::Digest;
        let mut hasher = sha2::Sha512::new();
        hasher.input(&payload_bytes);
        let payload_checksum: String = data_encoding::HEXUPPER.encode(&hasher.result()[..]);

        let mut transaction_header = sawtooth_sdk::messages::transaction::TransactionHeader::new();
        transaction_header.set_family_name(String::from("alica-messages"));
        transaction_header.set_family_version(String::from("0.1.0"));
        transaction_header.set_nonce(data_encoding::HEXUPPER.encode(&nonce));
        transaction_header.set_inputs(protobuf::RepeatedField::from_vec(vec![]));
        transaction_header.set_outputs(protobuf::RepeatedField::from_vec(vec![]));
        transaction_header.set_signer_public_key(
            signer
                .get_public_key()
                .expect("Error retreiving signer's public key")
                .as_hex(),
        );
        transaction_header.set_batcher_public_key(
            signer
                .get_public_key()
                .expect("Error retreiving signer's public key")
                .as_hex(),
        );
        transaction_header.set_payload_sha512(payload_checksum);

        use protobuf::Message;
        let transaction_header_bytes = transaction_header
            .write_to_bytes()
            .expect("Error serializing transaction header");

        let mut transaction = sawtooth_sdk::messages::transaction::Transaction::new();
        transaction.set_header_signature(
            signer
                .sign(&transaction_header_bytes)
                .expect("Error signing transaction header"),
        );
        transaction.set_header(transaction_header_bytes);
        transaction.set_payload(payload_bytes);

        let mut batch_header = sawtooth_sdk::messages::batch::BatchHeader::new();
        batch_header.set_signer_public_key(
            signer
                .get_public_key()
                .expect("Error retreiving signer's public key")
                .as_hex(),
        );
        batch_header.set_transaction_ids(protobuf::RepeatedField::from_vec(
            vec![transaction.clone()]
                .iter()
                .map(|t| String::from(t.get_header_signature()))
                .collect(),
        ));

        let batch_header_bytes = batch_header
            .write_to_bytes()
            .expect("Error serializing batch header");

        let mut batch = sawtooth_sdk::messages::batch::Batch::new();
        batch.set_header_signature(
            signer
                .sign(&batch_header_bytes)
                .expect("Error signing batch header"),
        );
        batch.set_header(batch_header_bytes);
        batch.set_transactions(protobuf::RepeatedField::from_vec(vec![transaction]));

        let mut batch_submit_request =
            sawtooth_sdk::messages::client_batch_submit::ClientBatchSubmitRequest::new();
        batch_submit_request.set_batches(protobuf::RepeatedField::from_vec(vec![batch]));

        let correlation_id = uuid::Uuid::new_v4().to_string();

        let connection =
            sawtooth_sdk::messaging::zmq_stream::ZmqMessageConnection::new(self.validator_url);
        use sawtooth_sdk::messaging::stream::{MessageConnection, MessageSender};
        let (sender, _receiver) = connection.create();
        match sender.send(
            sawtooth_sdk::messages::validator::Message_MessageType::CLIENT_BATCH_SUBMIT_REQUEST,
            correlation_id.as_str(),
            &batch_submit_request
                .write_to_bytes()
                .expect("Error serializing client batch submit request")[..],
        ) {
            Ok(mut future) => match future.get() {
                Ok(result) => println!(
                    "Got response of type {:?} with content {:?}",
                    result.get_message_type(),
                    result.get_content()
                ),
                Err(error) => panic!(
                    "Error unpacking response from batch submit request. Error was {}",
                    error
                ),
            },
            Err(error) => panic!("Error sending batch submit request. Error was {}", error),
        };
    }
}

#[derive(Debug)]
pub struct Payload {
    r#type: String,
    message: String,
}

impl Payload {
    pub fn new(r#type: String, message: String) -> Self {
        Payload { r#type, message }
    }

    pub fn serialize(&self) -> Vec<u8> {
        format!("{};;{}", self.r#type, self.message)
            .as_bytes()
            .to_vec()
    }
}
