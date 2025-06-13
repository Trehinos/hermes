use clap::{Arg, ArgAction, Command};
use hermes::client::Client;

/// Version of the `hermes` crate used to build this binary.
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let matches = Command::new("hermes-client")
        .version(VERSION)
        .disable_help_flag(false)
        .disable_version_flag(true)
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .action(ArgAction::Version)
                .help("Print version information"),
        )
        .arg(
            Arg::new("uri")
                .value_name("URI")
                .help("Destination URI")
                .required(true),
        )
        .get_matches();

    let url = matches
        .get_one::<String>("uri")
        .expect("URI argument missing");
    
    println!("Sending request to {}", url);
    let response = Client::get(url).await?;
    println!("{}", response);
    Ok(())
}
