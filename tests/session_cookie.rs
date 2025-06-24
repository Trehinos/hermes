use hermes::concepts::value::Value;
use hermes::concepts::Parsable;
use hermes::http::routing::router::{Route, Router};
use hermes::http::services::client::Client;
use hermes::http::services::server::{RequestContext, Server};
use hermes::http::session::FileStore;
use hermes::http::{
    Headers, MessageTrait, Method, Request, RequestFactory, ResponseFactory, Status, Version,
};

#[tokio::test]
#[ignore]
async fn test_session_persists_with_cookie() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);

    let dir = std::env::temp_dir().join("hermes_session_cookie");
    let store = FileStore::new(&dir);

    let mut router: Router<RequestContext<_>> = Router::new();
    router.add_route(Route::new(
        "/",
        vec![Method::Get],
        Headers::new(),
        Box::new(|ctx: &RequestContext<_>, _req: &mut Request| {
            let mut sess = ctx.session.borrow_mut();
            let n = match sess.get("count") {
                Some(Value::Int(i)) => *i,
                _ => 0,
            } + 1;
            sess.insert("count", Value::Int(n));
            let factory = ResponseFactory::version(Version::Http1_1);
            factory
                .with_status(Status::OK, Headers::new())
                .with_body(&n.to_string())
        }),
    ));

    let address = format!("127.0.0.1:{}", port);
    let server = Server::new(&address, router, store);
    let handle = tokio::spawn(async move {
        let _ = server.run().await;
    });
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let url = format!("http://{}", address);
    let (_, uri) = hermes::http::Uri::parse(&url).unwrap();
    let factory = RequestFactory::version(Version::Http1_1);
    let mut client = Client::new("127.0.0.1".into(), port);
    let req1 = factory.build(Method::Get, uri.clone(), Headers::new(), "");
    let resp1 = client.send(req1).await.unwrap();
    assert_eq!(resp1.body(), "1");
    let req2 = factory.build(Method::Get, uri, Headers::new(), "");
    let resp2 = client.send(req2).await.unwrap();
    assert_eq!(resp2.body(), "2");

    handle.abort();
    std::fs::remove_dir_all(&dir).ok();
}
