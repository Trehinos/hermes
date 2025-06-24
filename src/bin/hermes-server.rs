use clap::{value_parser, Arg, ArgAction, Command};
use hermes::http::routing::router::{Route, Router};
use hermes::http::services::server::{RequestContext, Server};
use hermes::http::session::FileStore;
use hermes::http::{Headers, Method, Request, ResponseFactory, Status, Version};

/// Version of the `hermes` crate used to build this binary.
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let matches = Command::new("hermes-server")
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
            Arg::new("address")
                .short('a')
                .long("address")
                .value_name("address")
                .default_value("0.0.0.0")
                .help("Address to bind"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("port")
                .default_value("80")
                .value_parser(value_parser!(u16))
                .help("Port to listen on"),
        )
        .get_matches();

    let address = matches
        .get_one::<String>("address")
        .expect("address has default");
    let port = matches.get_one::<u16>("port").expect("port has default");
    println!("Listening on {}:{}", address, port);
    let addr = format!("{}:{}", address, port);
    let store = FileStore::new("./sessions");
    let mut router: Router<RequestContext<_>> = Router::new();
    router.add_route(Route::new(
        "/",
        vec![Method::Get],
        Headers::new(),
        Box::new(|_ctx: &RequestContext<_>, _req: &mut Request| {
            ResponseFactory::version(Version::Http1_1).with_status(Status::OK, Headers::new())
        }),
    ));
    let server = Server::new(&addr, router, store);
    server.run().await
}
