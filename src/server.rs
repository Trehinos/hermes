use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::http::{Headers, Request, ResponseFactory, Status, Version};
use crate::concepts::Parsable;

/// Simple asynchronous TCP server handling HTTP requests.
pub struct Server {
    address: String,
}

impl Server {
    /// Create a new server bound to `address`.
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
        }
    }

    /// Start listening for connections and process them concurrently.
    pub async fn run(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.address).await?;
        loop {
            let (stream, _) = listener.accept().await?;
            tokio::spawn(async move {
                let _ = handle_connection(stream).await;
            });
        }
    }
}

async fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await?;
    let request = String::from_utf8_lossy(&buf);
    let _ = Request::parse(&request);

    let factory = ResponseFactory::version(Version::Http1_1);
    let response = factory.with_status(Status::NoContent, Headers::new());
    stream.write_all(response.to_string().as_bytes()).await?;
    stream.shutdown().await?;
    Ok(())
}
