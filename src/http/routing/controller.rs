//! Controller and middleware abstractions for routing.

use crate::concepts::BoxVec;
use crate::http::{Request, RequestTrait, Response, ResponseTrait};

/// Minimal request handler.
pub trait Controller<Ctx, Req: RequestTrait = Request, Res: ResponseTrait = Response>:
    Send
{
    /// Handle a request and generate a response.
    fn handle(&mut self, context: &Ctx, req: &mut Req) -> Res;
}

impl<Ctx, Req: RequestTrait, Res: ResponseTrait, F: Send> Controller<Ctx, Req, Res> for F
where
    F: FnMut(&Ctx, &mut Req) -> Res,
{
    fn handle(&mut self, context: &Ctx, req: &mut Req) -> Res {
        self(context, req)
    }
}

/// Middleware executed before or after a [`Controller`].
pub trait Middleware<Ctx, Req: RequestTrait = Request, Res: ResponseTrait = Response>:
    Send
{
    fn handle(
        &mut self,
        context: &Ctx,
        req: &mut Req,
        next: &mut dyn Controller<Ctx, Req, Res>,
    ) -> Res;
}

/// Adapter allowing plain functions to act as [`Controller`]s.
pub struct ControllerFn<Ctx, Req: RequestTrait = Request, Res: ResponseTrait = Response>(
    pub fn(&Ctx, &mut Req) -> Res,
);

impl<Ctx, Req: RequestTrait, Res: ResponseTrait> Controller<Ctx, Req, Res>
    for ControllerFn<Ctx, Req, Res>
{
    fn handle(&mut self, context: &Ctx, req: &mut Req) -> Res {
        (self.0)(context, req)
    }
}

/// Internal helper to iterate over the middleware list.
struct MiddlewareChain<'a, Ctx, Req: RequestTrait = Request, Res: ResponseTrait = Response> {
    before: &'a mut [Box<dyn Middleware<Ctx, Req, Res>>],
    after: &'a mut [Box<dyn Middleware<Ctx, Req, Res>>],
    controller: &'a mut dyn Controller<Ctx, Req, Res>,
}

impl<'a, Ctx, Req: RequestTrait, Res: ResponseTrait> Controller<Ctx, Req, Res>
    for MiddlewareChain<'a, Ctx, Req, Res>
{
    fn handle(&mut self, context: &Ctx, req: &mut Req) -> Res {
        execute_middleware_chain(self.before, self.controller, self.after, context, req)
    }
}

/// Recursively execute a slice of middleware, then the controller, followed by
/// after middleware.
fn execute_middleware_chain<Ctx, Req: RequestTrait, Res: ResponseTrait>(
    before: &mut [Box<dyn Middleware<Ctx, Req, Res>>],
    controller: &mut dyn Controller<Ctx, Req, Res>,
    after: &mut [Box<dyn Middleware<Ctx, Req, Res>>],
    context: &Ctx,
    req: &mut Req,
) -> Res {
    if let Some((first, rest)) = before.split_first_mut() {
        let mut next_chain = MiddlewareChain {
            before: rest,
            after,
            controller,
        };
        first.handle(context, req, &mut next_chain)
    } else if let Some((first, rest)) = after.split_first_mut() {
        let mut next_chain = MiddlewareChain {
            before: &mut [],
            after: rest,
            controller,
        };
        first.handle(context, req, &mut next_chain)
    } else {
        controller.handle(context, req)
    }
}

/// Chain of [`Middleware`] executed around a [`Controller`].
pub struct Mediator<Ctx, Req: RequestTrait = Request, Res: ResponseTrait = Response> {
    before: BoxVec<dyn Middleware<Ctx, Req, Res>>,
    after: BoxVec<dyn Middleware<Ctx, Req, Res>>,
    controller: Box<dyn Controller<Ctx, Req, Res>>,
}

impl<Ctx, Req: RequestTrait, Res: ResponseTrait> Mediator<Ctx, Req, Res> {
    /// Build a new [`Mediator`] from lists of middleware and a final controller.
    /// The before middleware is executed first, followed by the controller, then the after middleware.
    pub fn new(
        before: BoxVec<dyn Middleware<Ctx, Req, Res>>,
        controller: Box<dyn Controller<Ctx, Req, Res>>,
        after: BoxVec<dyn Middleware<Ctx, Req, Res>>,
    ) -> Self {
        Self {
            before,
            after,
            controller,
        }
    }

    /// Append middleware executed before the controller.
    pub fn with_before(mut self, middleware: Box<dyn Middleware<Ctx, Req, Res>>) -> Self {
        self.before.push(middleware);
        self
    }

    /// Append middleware executed after the controller.
    pub fn with_after(mut self, middleware: Box<dyn Middleware<Ctx, Req, Res>>) -> Self {
        self.after.push(middleware);
        self
    }

    /// Replace the controller executed at the end of the chain.
    pub fn set_controller(&mut self, controller: Box<dyn Controller<Ctx, Req, Res>>) {
        self.controller = controller;
    }
}

impl<Ctx, Req: RequestTrait, Res: ResponseTrait> Controller<Ctx, Req, Res>
    for Mediator<Ctx, Req, Res>
{
    fn handle(&mut self, context: &Ctx, req: &mut Req) -> Res {
        execute_middleware_chain(
            &mut self.before,
            self.controller.as_mut(),
            &mut self.after,
            context,
            req,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{
        Authority, Headers, Message, MessageTrait, Method, Path, Query, Request, Response,
        ResponseFactory, Status, Uri, Version,
    };

    fn request(method: Method, path: &str) -> Request {
        let uri = Uri::new(
            String::new(),
            Authority::default(),
            Path::new(path.to_string(), None),
            Query::new(),
            None,
        );
        Request {
            method,
            target: uri,
            message: Message::v1_1(Headers::new(), String::new()),
        }
    }

    #[test]
    fn test_controller() {
        let mut ctrl = ControllerFn(|_, _| {
            ResponseFactory::version(Version::Http1_1).with_status(Status::OK, Headers::new())
        });

        let mut req = request(Method::Get, "/");
        let resp = ctrl.handle(&(), &mut req);
        assert_eq!(resp.status(), Status::OK);
    }

    struct Before;
    impl Middleware<(), Request, Response> for Before {
        fn handle(
            &mut self,
            ctx: &(),
            req: &mut Request,
            next: &mut dyn Controller<(), Request, Response>,
        ) -> Response {
            req.headers_mut().add("X-Before", "1");
            next.handle(ctx, req)
        }
    }

    struct After;
    impl Middleware<(), Request, Response> for After {
        fn handle(
            &mut self,
            ctx: &(),
            req: &mut Request,
            next: &mut dyn Controller<(), Request, Response>,
        ) -> Response {
            let mut res = next.handle(ctx, req);
            res.headers_mut().add("X-After", "1");
            res
        }
    }

    #[test]
    fn test_middleware_chain() {
        let mut before: BoxVec<dyn Middleware<(), Request, Response>> =
            vec![Box::new(Before) as Box<dyn Middleware<_, _, _>>];
        let mut after: BoxVec<dyn Middleware<(), Request, Response>> =
            vec![Box::new(After) as Box<dyn Middleware<_, _, _>>];
        let mut ctrl = ControllerFn(|_, _| {
            ResponseFactory::version(Version::Http1_1).with_status(Status::OK, Headers::new())
        });

        let mut req = request(Method::Get, "/");
        let resp = execute_middleware_chain(&mut before, &mut ctrl, &mut after, &(), &mut req);
        assert!(req.has_header("X-Before"));
        assert!(resp.has_header("X-After"));
    }

    #[test]
    fn test_mediator() {
        let mut mediator = Mediator::new(
            vec![Box::new(Before) as Box<dyn Middleware<_, _, _>>],
            Box::new(ControllerFn(|_, _| {
                ResponseFactory::version(Version::Http1_1).with_status(Status::OK, Headers::new())
            })),
            vec![Box::new(After) as Box<dyn Middleware<_, _, _>>],
        );

        let mut req = request(Method::Get, "/");
        let resp = mediator.handle(&(), &mut req);
        assert!(req.has_header("X-Before"));
        assert!(resp.has_header("X-After"));
        assert_eq!(resp.status(), Status::OK);
    }

    #[test]
    fn test_after_order() {
        use std::sync::{Arc, Mutex};

        struct Log(Vec<&'static str>);

        struct LoggingBefore(Arc<Mutex<Log>>);
        impl Middleware<(), Request, Response> for LoggingBefore {
            fn handle(
                &mut self,
                ctx: &(),
                req: &mut Request,
                next: &mut dyn Controller<(), Request, Response>,
            ) -> Response {
                self.0.lock().unwrap().0.push("before");
                next.handle(ctx, req)
            }
        }

        struct LoggingAfter(Arc<Mutex<Log>>);
        impl Middleware<(), Request, Response> for LoggingAfter {
            fn handle(
                &mut self,
                ctx: &(),
                req: &mut Request,
                next: &mut dyn Controller<(), Request, Response>,
            ) -> Response {
                let res = next.handle(ctx, req);
                self.0.lock().unwrap().0.push("after");
                res
            }
        }

        struct LoggingCtrl(Arc<Mutex<Log>>);
        impl Controller<(), Request, Response> for LoggingCtrl {
            fn handle(&mut self, _c: &(), _r: &mut Request) -> Response {
                self.0.lock().unwrap().0.push("controller");
                ResponseFactory::version(Version::Http1_1).with_status(Status::OK, Headers::new())
            }
        }

        let log = Arc::new(Mutex::new(Log(Vec::new())));
        let mut mediator = Mediator::new(
            vec![Box::new(LoggingBefore(log.clone())) as Box<dyn Middleware<_, _, _>>],
            Box::new(LoggingCtrl(log.clone())),
            vec![Box::new(LoggingAfter(log.clone())) as Box<dyn Middleware<_, _, _>>],
        );

        let mut req = request(Method::Get, "/");
        let _ = mediator.handle(&(), &mut req);

        assert_eq!(log.lock().unwrap().0, ["before", "controller", "after"]);
    }
}
