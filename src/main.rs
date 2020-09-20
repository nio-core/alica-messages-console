use alica_messages_client::{get_commandline_arguments, TransactionSubmissionCommand,
                            SawtoothCommand, TransactionListCommand};
use alica_messages_client::communication::{AlicaMessage, Client};

fn alica_message_from(agent_id: &str, message_type: &str, message: &str, timestamp: &str)
    -> AlicaMessage {
    AlicaMessage::new(
        String::from(agent_id),
        String::from(message_type),
        String::from(message_text),
        String::from(timestamp),
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
                Some(a) => a,
                None => panic!("No parameters supplied for transaction addition")
            };
            
            let message = alica_message_from(
                args.value_of("agent id").unwrap(),
                args.value_of("message type").unwrap(),
                args.value_of("message").unwrap(),
                args.value_of("timestamp").unwrap()
            );

            std::boxed::Box::new(TransactionSubmissionCommand::new(&client, message))
        },
        "list" => {
            std::boxed::Box::new(TransactionListCommand::new(&client))
        },
        _ => panic!("Could not find subcommand")
    };

    command.execute();
}
