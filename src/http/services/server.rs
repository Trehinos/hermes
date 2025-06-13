use crate::concepts::Parsable;
use crate::http::{Headers, Request, ResponseFactory, Status, Version};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// Simple asynchronous TCP server handling HTTP requests.
///
/// # Examples
///
/// ```no_run
/// use hermes::http::services::server::Server;
///
/// # tokio_test::block_on(async {
/// let server = Server::new("127.0.0.1:8080");
/// // This will block forever handling incoming connections
/// // and therefore is marked as `no_run` in the documentation.
/// // server.run().await.unwrap();
/// # })
/// ```
#[derive(Clone)]
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
            let this = self.clone();
            tokio::spawn(async move {
                let _ = this.handle_connection(stream).await;
            });
        }
    }

    async fn handle_connection(&self, mut stream: TcpStream) -> std::io::Result<()> {
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).await?;
        let request = String::from_utf8_lossy(&buf);
        let _ = Request::parse(&request);

        let factory = ResponseFactory::version(Version::Http1_1);
        let mut headers = Headers::new();
        headers.insert("Content-Length", &["0".to_string()]);
        let response = factory.with_status(Status::NoContent, headers);
        stream.write_all(response.to_string().as_bytes()).await?;
        stream.shutdown().await?;
        Ok(())
    }
}
