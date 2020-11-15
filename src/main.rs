use alica_messages_client::get_commandline_arguments;
use alica_messages_client::command::SawtoothCommand;
use alica_messages_client::command::transaction;
use alica_messages_client::command::state;
use alica_messages_client::sawtooth::{self, AlicaMessage, TransactionFamily};
use alica_messages_client::sawtooth::factory::GeneralPurposeComponentFactory;

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

fn create_client_from<'a>(args: &clap::ArgMatches) -> sawtooth::Client<'a> {
    let validator_url = args.value_of("connect").expect("Validator address missing");
    let transaction_family = TransactionFamily::new("alica_messages", "0.1.0");
    let component_factory = GeneralPurposeComponentFactory::new(transaction_family);
    sawtooth::Client::new(validator_url, Box::from(component_factory))
}

fn alica_message_from(args: &clap::ArgMatches) -> AlicaMessage {
    AlicaMessage::new(
        args.value_of("agent id").expect("agent id missing").to_string(),
        args.value_of("message type").expect("message type missing").to_string(),
        args.value_of("message").expect("message missing").to_string(),
        args.value_of("timestamp").expect("timestamp missing").to_string()
    )
}
