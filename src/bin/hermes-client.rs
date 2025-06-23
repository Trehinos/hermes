use clap::{Arg, ArgAction, Command};
use hermes::concepts::Parsable;
use hermes::http::services::client::Client;
use hermes::http::{Headers, Method, RequestFactory, Uri, Version};

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
            Arg::new("header")
                .short('H')
                .long("header")
                .value_name("HEADER")
                .action(ArgAction::Append)
                .help("Add a header"),
        )
        .arg(
            Arg::new("method")
                .value_name("METHOD")
                .help("HTTP method")
                .required(true),
        )
        .arg(
            Arg::new("url")
                .value_name("URL")
                .help("Destination URL")
                .required(true),
        )
        .arg(
            Arg::new("body")
                .value_name("BODY")
                .help("Request body")
                .required(false),
        )
        .get_matches();

    let method_str = matches
        .get_one::<String>("method")
        .expect("method argument missing");
    let (_, method) = Method::parse(method_str)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid method"))?;

    let url = matches
        .get_one::<String>("url")
        .expect("URL argument missing");

    let body = matches
        .get_one::<String>("body")
        .cloned()
        .unwrap_or_default();

    let mut headers = Headers::new();
    let (_, uri) = Uri::parse(url)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid url"))?;
    headers.insert("Host", &[uri.authority.host.clone()]);

    if let Some(values) = matches.get_many::<String>("header") {
        for h in values {
            let (_, (name, vals)) = Headers::parse_header(h).map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid header")
            })?;
            headers.insert(&name, &vals);
        }
    }

    println!("Sending {} request to {}", method, url);
    let factory = RequestFactory::version(Version::Http1_1);
    let request = factory.build(method, uri, headers, &body);
    let host = request.target.authority.host.clone();
    let port = request.target.authority.port.unwrap_or(80);
    let mut client = Client::new(host, port).await;
    let response = client.send(request).await?;
    println!("{}", response);
    Ok(())
}
