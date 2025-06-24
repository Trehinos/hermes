use crate::concepts::Parsable;
use crate::http::cookie::{Cookie, CookieJar};
use crate::http::routing::router::Router;
use crate::http::session::{generate_id, Session, SessionStore};
use crate::http::{Headers, Request, ResponseFactory, Status, Version};
use std::cell::RefCell;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

/// Simple asynchronous TCP server handling HTTP requests.
///
/// # Examples
///
/// ```no_run
/// use hermes::http::services::server::{RequestContext, Server};
/// use hermes::http::routing::router::{Route, Router};
/// use hermes::http::session::FileStore;
/// use hermes::http::{Headers, Method, Request, ResponseFactory, Status, Version};
///
/// # tokio_test::block_on(async {
/// let store = FileStore::new("/tmp/sessions");
/// let mut router: Router<RequestContext<_>> = Router::new();
/// router.add_route(Route::new(
///     "/", vec![Method::Get], Headers::new(),
///     Box::new(|_ctx: &RequestContext<_>, _req: &mut Request| {
///         ResponseFactory::version(Version::Http1_1)
///             .with_status(Status::OK, Headers::new())
///     }),
/// ));
/// let server = Server::new("127.0.0.1:8080", router, store);
/// // This will block forever handling incoming connections
/// // and therefore is marked as `no_run` in the documentation.
/// // server.run().await.unwrap();
/// # })
/// ```
pub struct RequestContext<S: SessionStore + Clone> {
    pub session: RefCell<Session<S>>,
    pub cookies: RefCell<CookieJar>,
}

#[derive(Clone)]
pub struct Server<S: SessionStore + Clone> {
    address: String,
    router: Arc<Mutex<Router<RequestContext<S>>>>,
    store: S,
}

impl<S: SessionStore + Clone + Send + Sync + 'static> Server<S> {
    /// Create a new server bound to `address` with `router` and `store`.
    pub fn new(address: &str, router: Router<RequestContext<S>>, store: S) -> Self {
        Self {
            address: address.to_string(),
            router: Arc::new(Mutex::new(router)),
            store,
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
        let (_, mut req) = Request::parse(&request)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid request"))?;

        let mut jar = req.cookies();
        let mut new_id = false;
        let sid = if let Some(id) = jar.get("sid").cloned() {
            id
        } else {
            new_id = true;
            generate_id()
        };

        let session = Session::new(&sid, self.store.clone());
        let ctx = RequestContext {
            session: RefCell::new(session),
            cookies: RefCell::new(jar.clone()),
        };

        let mut router = self.router.lock().await;
        let mut response = router.handle_request(&ctx, &mut req).unwrap_or_else(|| {
            ResponseFactory::version(Version::Http1_1).with_status(Status::NotFound, Headers::new())
        });
        drop(router);

        ctx.session.borrow().persist();
        if new_id {
            response = response.with_cookie(Cookie::new("sid", sid));
        }

        stream.write_all(response.to_string().as_bytes()).await?;
        stream.shutdown().await?;
        Ok(())
    }
}
