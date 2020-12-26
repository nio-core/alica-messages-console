use alica_messages_client::{create_alica_message, get_or_create_keyfile, determine_key_file, create_signer, create_sawtooth_client, create_filters};
use alica_messages_client::command::{SawtoothCommand, batch};
use alica_messages_client::command::state;
use alica_messages_client::sawtooth::factory::GeneralPurposeComponentFactory;
use sawtooth_alica_payload::{TransactionFamily, payloads};

fn main() {
    let cli_definition = clap::load_yaml!("../cli.yml");
    let app = clap::App::from(cli_definition);
    let args = app.get_matches();

    let configured_key_file = get_or_create_keyfile(&args);
    let key_file = determine_key_file(configured_key_file);
    let signer = create_signer(&key_file);

    let payload_format = payloads::pipe_separated::Format::default();
    let transaction_family = TransactionFamily::new("alica_messages", &vec!["0.1.0".to_string()]);
    let component_factory = GeneralPurposeComponentFactory::new(&transaction_family, &payload_format, signer);

    let client = create_sawtooth_client(&args, &component_factory);

    let command: Box<dyn SawtoothCommand> = match args.subcommand() {
        ("batch", Some(args)) => match args.subcommand() {
            ("create", Some(args)) => Box::new(batch::CreateCommand::new(client, create_alica_message(&args))),
            ("", _) => panic!("No subcommand supplied to batch"),
            (cmd, _) => panic!("No subcommand {} exists for batch", cmd)
        },
        ("state", Some(args)) => match args.subcommand() {
            ("list", Some(args)) => {
                let filters = create_filters(&args);
                Box::new(state::ListCommand::new(client, &transaction_family.calculate_namespace(),
                                                 &payload_format, filters))
            },
            ("", _) => panic!("No subcommand supplied to state"),
            (cmd, _) => panic!("No subcommand {} exists for state", cmd),
        },
        ("", _) => panic!("No subcommand supplied"),
        (cmd, _) => panic!("No subcommand {} exists", cmd)
    };

    command.execute().expect("Command execution failed");
}
