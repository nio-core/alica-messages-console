pub mod communication;
pub mod helper;

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
                .help("ZeroMQ address of a validator"),
        )
        .arg(
            clap::Arg::with_name("agent id")
                .short("i")
                .long("id")
                .takes_value(true)
                .help("The unique identifier of the sending agent"),
        )
        .arg(
            clap::Arg::with_name("message type")
                .short("t")
                .long("type")
                .takes_value(true)
                .help("The type of the message to log"),
        )
        .arg(
            clap::Arg::with_name("message")
                .short("m")
                .long("message")
                .takes_value(true)
                .help("The message to log"),
        )
        .arg(
            clap::Arg::with_name("timestamp")
                .short("z")
                .long("timestamp")
                .takes_value(true)
                .help("The timestamp of the moment the message was recorded"),
        )
        .get_matches()
}
