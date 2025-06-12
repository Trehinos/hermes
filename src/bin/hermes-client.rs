use hermes::client::Client;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let url = std::env::args().nth(1).expect("URL required");
    let response = Client::get(&url).await?;
    println!("{}", response);
    Ok(())
}
