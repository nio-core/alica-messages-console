use crate::sawtooth::TransactionFamily;
use sawtooth_sdk::signing;
use std::path::{Path, PathBuf};
use std::{fs, env};
use crate::sawtooth::factory::GeneralPurposeComponentFactory;
use sawtooth_alica_message_transaction_payload::payloads;

pub mod sawtooth;
pub mod command;

pub fn alica_message_from(args: &clap::ArgMatches) -> payloads::TransactionPayload {
    payloads::TransactionPayload::new(
        args.value_of("agent_id").expect("agent id missing"),
        args.value_of("message_type").expect("message type missing"),
        args.value_of("message").expect("message missing").as_bytes(),
        args.value_of("timestamp").expect("timestamp missing")
            .parse::<u64>().expect("Timestamp is not an integer")
    )
}

pub fn create_client_from(args: &clap::ArgMatches) -> sawtooth::Client {
    let configured_key_file = get_key_file_from(args);
    let key_file = determine_key_file(configured_key_file);
    let signer = create_signer_from(&key_file);

    let payload_serializer = Box::from(payloads::pipe_separated::Format::default());
    let transaction_family = TransactionFamily::new("alica_messages", "0.1.0", payload_serializer);
    let component_factory = GeneralPurposeComponentFactory::new(transaction_family, signer);

    let validator_url = args.value_of("connect").expect("Validator address missing");
    sawtooth::Client::new(validator_url, Box::from(component_factory))
}

fn get_key_file_from(args: &clap::ArgMatches) -> Option<Box<Path>> {
    args.value_of("key file").map(|path| PathBuf::from(path).into_boxed_path())
}

fn determine_key_file(configured_path: Option<Box<Path>>) -> Box<Path> {
    let default_file_name = "sawtooth_key";
    let home_dir = dirs::home_dir();
    let current_dir = env::current_dir().expect("Invalid working directory");

    let mut default_path = home_dir.unwrap_or(current_dir);
    default_path.push(default_file_name);
    let default_path = default_path.into_boxed_path();

    configured_path.unwrap_or(default_path)
}

fn create_signer_from<'a>(path: &Box<Path>) -> signing::Signer<'a> {
    let private_key = if path.exists() {
        create_private_key_from_file(path)
    } else {
        let private_key = create_new_private_key();
        write_private_key_to_file(&private_key, path);
        private_key
    };

    let context = create_context_for_private_key(&private_key);

    signing::Signer::new_boxed(context, private_key)
}

fn create_private_key_from_file(path: &Box<Path>) -> Box<dyn signing::PrivateKey> {
    println!("Using key file at {}", path.to_str().expect("Could not display key file"));
    let raw_private_key = fs::read_to_string(path.to_str().expect("Invalid key file")).expect("Invalid key file");
    let private_key = signing::secp256k1::Secp256k1PrivateKey::from_hex(&raw_private_key)
        .expect("Private Key is not hex");
    Box::from(private_key)
}

fn write_private_key_to_file(private_key: &Box<dyn signing::PrivateKey>, path: &Box<Path>) {
    let key_contents = private_key.as_hex();
    println!("Creating key file at {}", path.to_str().expect("Could not display key file"));
    fs::write(path, key_contents.as_bytes()).expect("Could not write key file");
}

fn create_context_for_private_key(private_key: &Box<dyn signing::PrivateKey>) -> Box<dyn signing::Context>{
    signing::create_context(private_key.get_algorithm_name())
        .expect("This can not happen because the algorithm name is determined via the private key and is thus always valid")
}

fn create_new_private_key() -> Box<dyn signing::PrivateKey> {
    let context = signing::create_context("secp256k1")
        .expect("This happens only if the sawtooth team has decided to rename or remove the secp256k1 algorithm");
    context.new_random_private_key().expect("Could not create new private key")
}
