use alica_messages_client::{get_commandline_arguments, create_client_from, alica_message_from};
use alica_messages_client::command::SawtoothCommand;
use alica_messages_client::command::transaction;
use alica_messages_client::command::state;

fn main() {
    let args = get_commandline_arguments();
    let client = create_client_from(&args);
    let command: Box<dyn SawtoothCommand> = match args.subcommand() {
        ("new", Some(args)) => Box::new(transaction::SubmissionCommand::new(&client, alica_message_from(&args))),
        ("list", None) => Box::new(transaction::ListCommand::new(&client)),
        ("state", None) => Box::new(state::ListCommand::new(&client, "")),
        _ => panic!("No subcommand supplied")
    };

    command.execute().expect("Command execution failed");
}
