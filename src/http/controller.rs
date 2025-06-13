use crate::http::{Request, Response};

/// Trait for minimal request handlers.
pub trait Controller: Send {
    /// Handle `req` and generate a [`Response`].
    fn handle(&mut self, req: Request) -> Response;
}

impl<F> Controller for F
where
    F: FnMut(Request) -> Response + Send,
{
    fn handle(&mut self, req: Request) -> Response {
        self(req)
    }
}
