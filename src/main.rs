use alica_messages_client::{create_alica_message, sawtooth, get_or_create_keyfile, determine_key_file, create_signer};
use alica_messages_client::command::{SawtoothCommand, batch};
use alica_messages_client::command::transaction;
use alica_messages_client::command::state;
use alica_messages_client::sawtooth::factory::GeneralPurposeComponentFactory;
use sawtooth_alica_message_transaction_payload::{TransactionFamily, payloads};

fn main() {
    let cli_definition = clap::load_yaml!("../cli.yml");
    let app = clap::App::from(cli_definition);
    let args = app.get_matches();

    let configured_key_file = get_or_create_keyfile(&args);
    let key_file = determine_key_file(configured_key_file);
    let signer = create_signer(&key_file);

    let payload_format = Box::from(payloads::pipe_separated::Format::default());
    let transaction_family = TransactionFamily::new("alica_messages", &vec!["0.1.0".to_string()]);
    let component_factory = GeneralPurposeComponentFactory::new(&transaction_family, payload_format, signer);

    let validator_url = args.value_of("connect").expect("Validator address missing");
    let client = sawtooth::Client::new(validator_url, &component_factory);

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
            Some("list") => Box::new(state::ListCommand::new(client, &transaction_family.calculate_namespace())),
            Some(cmd) => panic!("No subcommand {} exists for state", cmd),
            None => panic!("No subcommand supplied for state")
        },
        (cmd, _) => panic!("No subcommand {} exists", cmd)
    };

    command.execute().expect("Command execution failed");
}
