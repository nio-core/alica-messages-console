use alica_messages_client::{create_sawtooth_client, create_alica_message};
use alica_messages_client::command::{SawtoothCommand, batch};
use alica_messages_client::command::transaction;
use alica_messages_client::command::state;

fn main() {
    let cli_definition = clap::load_yaml!("../cli.yml");
    let app = clap::App::from(cli_definition);
    let args = app.get_matches();

    let client = create_sawtooth_client(&args);
    let command: Box<dyn SawtoothCommand> = match args.subcommand() {
        ("batch", Some(args)) => match args.subcommand() {
            ("create", Some(args)) => Box::new(batch::CreateCommand::new(client, create_alica_message(&args))),
            (cmd, _) => panic!("No subcommand {} exists for batch", cmd)
        },
        ("transaction", Some(args)) => match args.subcommand_name() {
            Some("list") => Box::new(transaction::ListCommand::new(client)),
            Some(cmd) => panic!("No subcommand {} exists for transaction", cmd),
            None => panic!("No subcommand supplied for transaction")
        },
        ("state", Some(args)) => match args.subcommand_name() {
            Some("list") => Box::new(state::ListCommand::new(client, "")),
            Some(cmd) => panic!("No subcommand {} exists for state", cmd),
            None => panic!("No subcommand supplied for state")
        },
        (cmd, _) => panic!("No subcommand {} exists", cmd)
    };

    command.execute().expect("Command execution failed");
}
