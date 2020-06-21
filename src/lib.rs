use protobuf::Message;
use rand::Rng;
use sawtooth_sdk::messaging::stream::{MessageConnection, MessageSender};
use sha2::Digest;

pub struct Client<'a> {
    validator_url: &'a str,
    fammily_name: String,
}

impl<'a> Client<'a> {
    pub fn new(url: &'a str) -> Self {
        Client {
            validator_url: url,
            fammily_name: String::from("alica_messages"),
        }
    }

    pub fn send(&self, message: AlicaMessage) {
        let context = sawtooth_sdk::signing::create_context("secp256k1")
            .expect("Invalid algorithm name in context cration");
        let private_key = context
            .new_random_private_key()
            .expect("error creating a private key");
        let crypto_factory = sawtooth_sdk::signing::CryptoFactory::new(context.as_ref());
        let signer = crypto_factory.new_signer(private_key.as_ref());

        let payload_bytes = message.serialize();

        let mut nonce = [0u8, 16];
        rand::thread_rng()
            .try_fill(&mut nonce)
            .expect("Error filling nonce");

        let mut hasher = sha2::Sha512::new();
        hasher.update(&payload_bytes);
        let payload_checksum: String = data_encoding::HEXLOWER.encode(&hasher.finalize()[..]);

        let address = self.state_address_for(&self.fammily_name, &message);

        let mut transaction_header = sawtooth_sdk::messages::transaction::TransactionHeader::new();
        transaction_header.set_family_name(String::from(self.fammily_name.clone()));
        transaction_header.set_family_version(String::from("0.1.0"));
        transaction_header.set_nonce(data_encoding::HEXLOWER.encode(&nonce));
        transaction_header.set_inputs(protobuf::RepeatedField::from_vec(vec![address.clone()]));
        transaction_header.set_outputs(protobuf::RepeatedField::from_vec(vec![address.clone()]));
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
        let (mut sender, _receiver) = connection.create();
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

        sender.close();
    }

    fn state_address_for(&self, family_name: &str, message: &AlicaMessage) -> String {
        let mut hasher = sha2::Sha512::new();
        hasher.update(format!(
            "{}{}{}",
            &message.agent_id, &message.message_type, &message.timestamp
        ));
        let payload_part = data_encoding::HEXLOWER.encode(&hasher.finalize());

        let mut hasher = sha2::Sha512::new();
        hasher.update(family_name);
        let namespace_part = data_encoding::HEXLOWER.encode(&hasher.finalize()[..]);

        format!("{}{}", &namespace_part[..6], &payload_part[..64])
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
            agent_id: agent_id,
            message_type: message_type,
            message: message,
            timestamp: timestamp,
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

pub fn get_commandline_arguments<'a>() -> clap::ArgMatches<'a> {
    clap::App::new("alica-messages")
        .about("Client for the alica-message transaction processor")
        .author("Sven Starcke")
        .version("0.1.0")
        .arg(
            clap::Arg::with_name("connect")
                .short("C")
                .long("connect")
                .takes_value(true)
                .help("ZeroMQ address of a validator"),
        )
        .arg(
            clap::Arg::with_name("agent id")
                .short("i")
                .long("id")
                .takes_value(true)
                .help("The unique identifier of the sending agent"),
        )
        .arg(
            clap::Arg::with_name("message type")
                .short("t")
                .long("type")
                .takes_value(true)
                .help("The type of the message to log"),
        )
        .arg(
            clap::Arg::with_name("message")
                .short("m")
                .long("message")
                .takes_value(true)
                .help("The message to log"),
        )
        .arg(
            clap::Arg::with_name("timestamp")
                .short("z")
                .long("timestamp")
                .takes_value(true)
                .help("The timestamp of the moment the message was recorded"),
        )
        .get_matches()
}
