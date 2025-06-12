use hermes::server::Server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::args().nth(1).unwrap_or_else(|| "80".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let server = Server::new(&addr);
    server.run().await
}
