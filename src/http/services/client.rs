use crate::concepts::value::json::JsonFormatter;
use crate::concepts::value::{Value, ValueFormatter};
use crate::concepts::Parsable;
use crate::http::{
    cookie::CookieJar, Headers, MessageTrait, Method, Request, RequestFactory, Response, Uri,
    Version,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// Minimal asynchronous HTTP client.
///
/// # Examples
///
/// ```no_run
/// use hermes::http::services::client::Client;
/// use hermes::http::ResponseTrait;
/// # tokio_test::block_on(async {
/// let response = Client::get("http://example.com").await.unwrap();
/// println!("{}", response.code());
/// # })
/// ```
pub struct Client {
    host: String,
    port: u16,
    cookies: CookieJar,
}

impl Client {
    pub fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
            cookies: CookieJar::new(),
        }
    }

    /// Access the stored cookies.
    pub fn cookies(&self) -> &CookieJar {
        &self.cookies
    }

    /// Mutable access to the stored cookies.
    pub fn cookies_mut(&mut self) -> &mut CookieJar {
        &mut self.cookies
    }

    async fn connect(&self) -> std::io::Result<TcpStream> {
        TcpStream::connect(format!("{}:{}", self.host, self.port)).await
    }

    /// Send a [`Request`] over the wire and return the parsed [`Response`].
    pub async fn send(&mut self, mut request: Request) -> std::io::Result<Response> {
        if self.cookies.iter().next().is_some() {
            request
                .headers_mut()
                .add("Cookie", &self.cookies.to_header());
        }
        let mut stream = self.connect().await?;
        stream.write_all(request.to_string().as_bytes()).await?;
        stream.shutdown().await?;
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).await?;
        let text = String::from_utf8_lossy(&buf);
        let (_, response) = Response::parse(&text).map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid response")
        })?;
        if let Some(values) = response.headers().get("Set-Cookie") {
            for v in values {
                let cookie = v.split(';').next().unwrap_or("");
                let jar = CookieJar::parse(cookie);
                for (n, val) in jar.iter() {
                    self.cookies.insert(n.clone(), val.clone());
                }
            }
        }
        Ok(response)
    }

    /// Convenience helper to perform a `method` request to `url` with specified `headers` and `body`.
    pub async fn request(
        method: Method,
        url: &str,
        headers: Headers,
        body: &str,
    ) -> std::io::Result<Response> {
        let (_, uri) = Uri::parse(url)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid url"))?;
        let mut headers = headers;
        headers.insert("Host", &[uri.authority.host.clone()]);
        let factory = RequestFactory::version(Version::Http1_1);
        let request = factory.build(method, uri, headers, body);
        let host = request.target.authority.host.clone();
        let port = request.target.authority.port.unwrap_or(80);
        let mut client = Self::new(host, port);
        client.send(request).await
    }

    /// Convenience helper to perform a HEAD request to `url`.
    pub async fn head(url: &str) -> std::io::Result<Response> {
        Self::request(Method::Head, url, Headers::new(), "").await
    }

    /// Convenience helper to perform a GET request to `url`.
    pub async fn get(url: &str) -> std::io::Result<Response> {
        Self::request(Method::Get, url, Headers::new(), "").await
    }

    /// Convenience helper to perform a POST request to `url` with specified `headers` and `body`.
    pub async fn post(url: &str, headers: Headers, body: &str) -> std::io::Result<Response> {
        Self::request(Method::Post, url, headers, body).await
    }

    /// Convenience helper to perform a PUT request to `url` with specified `headers` and `body`.
    pub async fn put(url: &str, headers: Headers, body: &str) -> std::io::Result<Response> {
        Self::request(Method::Put, url, headers, body).await
    }

    /// Convenience helper to perform a PATCH request to `url` with specified `headers` and `body`.
    pub async fn patch(url: &str, headers: Headers, body: &str) -> std::io::Result<Response> {
        Self::request(Method::Patch, url, headers, body).await
    }

    /// Convenience helper to perform a DELETE request to `url`.
    pub async fn delete(url: &str) -> std::io::Result<Response> {
        Self::request(Method::Delete, url, Headers::new(), "").await
    }

    pub async fn request_with_json(
        method: Method,
        url: &str,
        headers: Headers,
        body: Value,
    ) -> std::io::Result<Response> {
        Self::request(method, url, headers, &JsonFormatter.format(body)).await
    }
}
