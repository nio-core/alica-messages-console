pub mod communication;
pub mod sawtooth;
pub mod helper;
pub mod command;

pub fn get_commandline_arguments<'a>() -> clap::ArgMatches<'a> {
    clap::App::new("alica-messages")
        .about("Client for the alica-message transaction processor")
        .author("Sven Starcke")
        .version("0.1.0")
        .arg(
            clap::Arg::with_name("connect")
                .short("C")
                .long("connect")
                .takes_value(true)
                .required(true)
                .help("ZeroMQ address of a validator"),
        )
        .subcommand(clap::SubCommand::with_name("new")
            .help("Adds new transaction to the chain")
            .arg(
                clap::Arg::with_name("agent id")
                    .short("i")
                    .long("id")
                    .takes_value(true)
                    .required(true)
                    .help("The unique identifier of the sending agent"),
            )
            .arg(
                clap::Arg::with_name("message type")
                    .short("t")
                    .long("type")
                    .takes_value(true)
                    .required(true)
                    .help("The type of the message to log"),
            )
            .arg(
                clap::Arg::with_name("message")
                    .short("m")
                    .long("message")
                    .takes_value(true)
                    .required(true)
                    .help("The message to log"),
            )
            .arg(
                clap::Arg::with_name("timestamp")
                    .short("z")
                    .long("timestamp")
                    .required(true)
                    .takes_value(true)
                    .help("The timestamp of the moment the message was recorded"),
            ))
        .subcommand(clap::SubCommand::with_name("list")
            .help("Lists all transaction in the blockchain")
        )
        .subcommand(clap::SubCommand::with_name("state")
            .help("List all state entries in the blockchain")
        )
        .get_matches()
}
