use hermes::container::Container;
use hermes::http::routing::router::{Route, Router};
use hermes::http::{
    Authority, Headers, Message, MessageTrait, Method, Path, Query, Request, ResponseFactory,
    Status, Uri, Version,
};

#[derive(Clone)]
struct Config {
    name: &'static str,
}

#[test]
fn controller_uses_container() {
    let mut container = Container::new();
    container.register_named("conf", Config { name: "hermes" });

    let mut router: Router<Container> = Router::new();
    router.add_route(Route::new(
        "/conf",
        vec![Method::Get],
        Headers::new(),
        Box::new(|ctx: &Container, _req: &mut Request| {
            let cfg = ctx.resolve_named::<Config>("conf").unwrap();
            ResponseFactory::version(Version::Http1_1)
                .with_status(Status::OK, Headers::new())
                .with_body(cfg.name)
        }),
    ));

    let uri = Uri::new(
        String::new(),
        Authority::default(),
        Path::new("/conf".into(), None),
        Query::new(),
        None,
    );
    let mut req = Request {
        method: Method::Get,
        target: uri,
        message: Message::v1_1(Headers::new(), String::new()),
    };

    let resp = router.handle_request(&container, &mut req).unwrap();
    assert_eq!(resp.body(), "hermes");
}
