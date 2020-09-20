pub mod communication;
pub mod helper;

use sawtooth_sdk::messages::validator::Message_MessageType::CLIENT_BATCH_SUBMIT_REQUEST;
use sawtooth_sdk::messages::client_batch_submit::ClientBatchSubmitRequest;

use crate::communication::AlicaMessage;

pub trait SawtoothCommand {
    fn execute(&self);
}

pub struct TransactionSubmissionCommand<'a> {
    client: &'a communication::Client<'a>,
    message: &'a AlicaMessage
}

impl<'a> TransactionSubmissionCommand<'a> {
    pub fn new(client: &'a communication::Client, message: &'a AlicaMessage) -> Self {
        TransactionSubmissionCommand {
            client,
            message
        }
    }
}

impl<'a> SawtoothCommand for TransactionSubmissionCommand<'a> {
    fn execute(&self) {
        let transaction = self.client.transaction_for(self.message);
        let transactions = vec![transaction];
        let batch = self.client.batch_for(&transactions);

        let mut batch_submit_request = ClientBatchSubmitRequest::new();
        batch_submit_request.set_batches(protobuf::RepeatedField::from_vec(vec![batch]));

        self.client.send(&batch_submit_request, CLIENT_BATCH_SUBMIT_REQUEST);
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
                .required(true)
                .help("ZeroMQ address of a validator"),
        )
        .subcommand(clap::SubCommand::with_name("new")
            .help("Adds new transaction to the chain")
            .arg(
                clap::Arg::with_name("agent id")
                    .short("i")
                    .long("id")
                    .takes_value(true)
                    .required(true)
                    .help("The unique identifier of the sending agent"),
            )
            .arg(
                clap::Arg::with_name("message type")
                    .short("t")
                    .long("type")
                    .takes_value(true)
                    .required(true)
                    .help("The type of the message to log"),
            )
            .arg(
                clap::Arg::with_name("message")
                    .short("m")
                    .long("message")
                    .takes_value(true)
                    .required(true)
                    .help("The message to log"),
            )
            .arg(
                clap::Arg::with_name("timestamp")
                    .short("z")
                    .long("timestamp")
                    .required(true)
                    .takes_value(true)
                    .help("The timestamp of the moment the message was recorded"),
            ))
        .get_matches()
}
