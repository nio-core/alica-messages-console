use alica_messages_client::get_commandline_arguments;
use alica_messages_client::communication::{AlicaMessage, Client};
use alica_messages_client::commands::{SawtoothCommand, TransactionSubmissionCommand,
                                      TransactionListCommand};

fn alica_message_from(args: &clap::ArgMatches) -> AlicaMessage {
    AlicaMessage::new(
        args.value_of("agent id").unwrap().to_string(),
        args.value_of("message type").unwrap().to_string(),
        args.value_of("message").unwrap().to_string(),
        args.value_of("timestamp").unwrap().to_string()
    )
}

fn main() {
    let args = get_commandline_arguments();
    let validator_url = args.value_of("connect").unwrap();
    let client = Client::new(String::from(validator_url));
    let (subcommand, subcommand_args) = args.subcommand();

    let command: Box<dyn SawtoothCommand> = match subcommand {
        "new" => {
            let args = match subcommand_args {
                Some(args) => args,
                None => panic!("No parameters supplied for transaction addition")
            };

            std::boxed::Box::new(TransactionSubmissionCommand::new(&client, alica_message_from(args)))
        },
        "list" => {
            std::boxed::Box::new(TransactionListCommand::new(&client))
        },
        _ => panic!("Could not find subcommand")
    };

    command.execute();
}
