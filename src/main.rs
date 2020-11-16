use alica_messages_client::{create_client_from, alica_message_from};
use alica_messages_client::command::SawtoothCommand;
use alica_messages_client::command::transaction;
use alica_messages_client::command::state;

fn main() {
    let cli_definition = clap::load_yaml!("../cli.yml");
    let app = clap::App::from(cli_definition);
    let args = app.get_matches();

    let client = create_client_from(&args);
    let command: Box<dyn SawtoothCommand> = match args.subcommand() {
        ("batch", Some(args)) => match args.subcommand() {
            ("create", Some(args)) => Box::new(transaction::SubmissionCommand::new(&client, alica_message_from(&args))),
            (cmd, _) => panic!("No subcommand {} exists for batch", cmd)
        },
        ("transaction", Some(args)) => match args.subcommand_name() {
            Some("list") => Box::new(transaction::ListCommand::new(&client)),
            Some(cmd) => panic!("No subcommand {} exists for transaction", cmd),
            None => panic!("No subcommand supplied for transaction")
        },
        ("state", Some(args)) => match args.subcommand_name() {
            Some("list") => Box::new(state::ListCommand::new(&client, "")),
            Some(cmd) => panic!("No subcommand {} exists for state", cmd),
            None => panic!("No subcommand supplied for state")
        },
        (cmd, _) => panic!("No subcommand {} exists", cmd)
    };

    command.execute().expect("Command execution failed");
}
