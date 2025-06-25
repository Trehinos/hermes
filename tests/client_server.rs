use hermes::http::services::client::Client;
use hermes::http::services::server::Server;
use hermes::http::{ResponseTrait, Status};

/// Ensure the client can talk to the server and multiple requests are
/// handled concurrently.
#[tokio::test]
async fn test_client_server_parallel_requests() {
    // Bind to port 0 to obtain a free port
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);

    let address = format!("127.0.0.1:{}", port);
    let server = Server::new(&address);
    let handle = tokio::spawn(async move {
        // Ignore the result, the task will be aborted at the end of the test
        let _ = server.run().await;
    });

    // Give the server a moment to start listening
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let url = format!("http://{}", address);
    let url_with_slash = format!("{}/", url);
    let mut tasks = Vec::new();
    for _ in 0..5 {
        let u = url.clone();
        tasks.push(tokio::spawn(async move { Client::get(&u).await.unwrap() }));
    }
    // Also request the same endpoint including a trailing slash.
    for _ in 0..5 {
        let u = url_with_slash.clone();
        tasks.push(tokio::spawn(async move { Client::get(&u).await.unwrap() }));
    }

    for t in tasks {
        let resp = t.await.unwrap();
        assert_eq!(resp.status(), Status::NoContent);
    }

    handle.abort();
}
