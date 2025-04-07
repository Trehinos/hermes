use crate::framework::server::ServerRequest;
use crate::http::Response;

pub trait Handler {
    fn check(&self, _: &ServerRequest) -> bool {
        true
    }
    fn handle(&mut self, request: &ServerRequest) -> Response;
}

pub trait MiddlewareTrait: Handler {
    fn process(&mut self, other: &mut dyn Handler, request: &ServerRequest) -> Response {
        if self.check(request) {
            self.handle(request)
        } else {
            other.handle(request)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Controller<H, A>
where
    H: FnOnce(&ServerRequest) -> Response,
    A: FnOnce(&ServerRequest) -> bool,
{
    handle: H,
    accept: A,
}

impl<H, A> Controller<H, A>
where
    H: FnOnce(&ServerRequest) -> Response,
    A: FnOnce(&ServerRequest) -> bool,
{
    pub fn new(handle: H, accept: A) -> Self {
        Self { handle, accept }
    }
}

impl<H: Clone, A: Clone> Handler for Controller<H, A>
where
    H: FnOnce(&ServerRequest) -> Response,
    A: FnOnce(&ServerRequest) -> bool,
{
    fn check(&self, request: &ServerRequest) -> bool {
        self.accept.clone()(request)
    }
    fn handle(&mut self, request: &ServerRequest) -> Response {
        self.handle.clone()(request)
    }
}
