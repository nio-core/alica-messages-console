use alica_messages_client::get_commandline_arguments;
use alica_messages_client::command::SawtoothCommand;
use alica_messages_client::command::transaction;
use alica_messages_client::command::state;
use alica_messages_client::sawtooth::{AlicaMessage, Client};

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
    let client = Client::new(validator_url);
    let (subcommand, subcommand_args) = args.subcommand();

    let command: Box<dyn SawtoothCommand> = match subcommand {
        "new" => {
            let args = match subcommand_args {
                Some(args) => args,
                None => {
                    println!("No parameters supplied for transaction addition");
                    return;
                }
            };

            Box::new(transaction::SubmissionCommand::new(&client, alica_message_from(args)))
        },
        "list" => {
            Box::new(transaction::ListCommand::new(&client))
        },
        "state" => {
            Box::new(state::ListCommand::new(&client))
        },
        _ => {
            println!("No subcommand supplied");
            return;
        }
    };

    command.execute().expect("Command execution failed");
}
