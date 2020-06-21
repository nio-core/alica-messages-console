use ::alica_messages_client::{get_commandline_arguments, AlicaMessage, Client};

fn main() {
    let args = get_commandline_arguments();

    let validator_url = match args.value_of("connect") {
        Some(url) => url,
        None => panic!("Missing validator address!"),
    };

    let agent_id = match args.value_of("agent id") {
        Some(id) => id,
        None => panic!("Missing agent id"),
    };

    let message_type = match args.value_of("message type") {
        Some(message_type) => message_type,
        None => panic!("Missing message type"),
    };

    let message_text = match args.value_of("message") {
        Some(message) => message,
        None => panic!("Missing message"),
    };

    let timestamp = match args.value_of("timestamp") {
        Some(timestamp) => timestamp,
        None => panic!("Missing timestamp"),
    };

    let message = AlicaMessage::new(
        String::from(agent_id),
        String::from(message_type),
        String::from(message_text),
        String::from(timestamp),
    );

    let client = Client::new(validator_url);
    client.send(message);
}
