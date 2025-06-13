use crate::concepts::Parsable;
use crate::http::{Headers, Request, Response, Version};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// Minimal asynchronous HTTP client.
///
/// # Examples
///
/// ```no_run
/// use hermes::client::Client;
/// use hermes::http::ResponseTrait;
/// # tokio_test::block_on(async {
/// let response = Client::get("http://example.com").await.unwrap();
/// println!("{}", response.code());
/// # })
/// ```
pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub async fn new(host: String, port: u16) -> Self {
        Self {
            stream: TcpStream::connect(format!("{}:{}", host, port))
                .await
                .unwrap(),
        }
    }

    pub async fn send(&mut self, request: Request) -> std::io::Result<Response> {
        self.stream
            .write_all(request.to_string().as_bytes())
            .await?;
        self.stream.shutdown().await?;
        let mut buf = Vec::new();
        self.stream.read_to_end(&mut buf).await?;
        let text = String::from_utf8_lossy(&buf);
        let (_, response) = Response::parse(&text).map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid response")
        })?;
        Ok(response)
    }

    /// Convenience helper to perform a GET request to `url`.
    pub async fn get(url: &str) -> std::io::Result<Response> {
        let (_, uri) = crate::http::Uri::parse(url)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid url"))?;
        let mut headers = Headers::new();
        headers.insert("Host", &[uri.authority.host.clone()]);
        let factory = crate::http::RequestFactory::version(Version::Http1_1);
        let request = factory.get(uri, headers);
        let host = request.target.authority.host.clone();
        let port = request.target.authority.port.unwrap_or(80);
        let mut client = Self::new(host, port).await;
        client.send(request).await
    }
}
