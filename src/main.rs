pub mod alica;

fn main() {
    let args = clap::App::new("alica-messages")
        .about("Client for the alica-message transaction processor")
        .author("Sven Starcke")
        .version("0.1.0")
        .arg(
            clap::Arg::with_name("connect")
                .short("C")
                .long("connect")
                .takes_value(true)
                .help("ZeroMQ address of a validator"),
        )
        .arg(
            clap::Arg::with_name("message")
                .short("m")
                .long("message")
                .takes_value(true)
                .help("The message to log"),
        )
        .get_matches();

    let validator_url = match args.value_of("connect") {
        Some(url) => url,
        None => panic!("Missing validator address!"),
    };

    let message = match args.value_of("message") {
        Some(message) => message,
        None => panic!("Missing message"),
    };

    let client = alica::messages::Client::new(validator_url);
    client.send(message);
}
