use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::http::{Headers, Request, Response, Version};
use crate::concepts::Parsable;

/// Minimal asynchronous HTTP client.
pub struct Client;

impl Client {
    /// Send a [`Request`] over TCP and parse the [`Response`].
    pub async fn send(request: Request) -> std::io::Result<Response> {
        let host = request.target.authority.host.clone();
        let port = request.target.authority.port.unwrap_or(80);
        let addr = format!("{}:{}", host, port);
        let mut stream = TcpStream::connect(addr).await?;
        stream.write_all(request.to_string().as_bytes()).await?;
        stream.shutdown().await?;
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).await?;
        let text = String::from_utf8_lossy(&buf);
        let (_, response) = Response::parse(&text).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid response"))?;
        Ok(response)
    }

    /// Convenience helper to perform a GET request to `url`.
    pub async fn get(url: &str) -> std::io::Result<Response> {
        let (_, uri) = crate::http::Uri::parse(url).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid url"))?;
        let mut headers = Headers::new();
        headers.insert("Host", &[uri.authority.host.clone()]);
        let factory = crate::http::RequestFactory::version(Version::Http1_1);
        let request = factory.get(uri, headers);
        Self::send(request).await
    }
}
