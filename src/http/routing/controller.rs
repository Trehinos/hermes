use crate::concepts::BoxVec;
use crate::http::{RequestTrait, ResponseTrait};

pub trait Controller<Ctx, Req: RequestTrait, Res: ResponseTrait>: Send {
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

pub trait Middleware<Ctx, Req: RequestTrait, Res: ResponseTrait>: Send {
    fn handle(
        &mut self,
        context: &Ctx,
        req: &mut Req,
        next: &mut dyn Controller<Ctx, Req, Res>,
    ) -> Res;
}

pub struct ControllerFn<Ctx, Req: RequestTrait, Res: ResponseTrait>(fn(&Ctx, &mut Req) -> Res);

impl<Ctx, Req: RequestTrait, Res: ResponseTrait> Controller<Ctx, Req, Res>
    for ControllerFn<Ctx, Req, Res>
{
    fn handle(&mut self, context: &Ctx, req: &mut Req) -> Res {
        (self.0)(context, req)
    }
}

struct MiddlewareChain<'a, Ctx, Req: RequestTrait, Res: ResponseTrait> {
    middlewares: &'a mut [Box<dyn Middleware<Ctx, Req, Res>>],
    controller: &'a mut dyn Controller<Ctx, Req, Res>,
}

impl<'a, Ctx, Req: RequestTrait, Res: ResponseTrait> Controller<Ctx, Req, Res>
    for MiddlewareChain<'a, Ctx, Req, Res>
{
    fn handle(&mut self, context: &Ctx, req: &mut Req) -> Res {
        execute_middleware_chain(self.middlewares, self.controller, context, req)
    }
}

fn execute_middleware_chain<Ctx, Req: RequestTrait, Res: ResponseTrait>(
    middlewares: &mut [Box<dyn Middleware<Ctx, Req, Res>>],
    controller: &mut dyn Controller<Ctx, Req, Res>,
    context: &Ctx,
    req: &mut Req,
) -> Res {
    if let Some((first, rest)) = middlewares.split_first_mut() {
        let mut next_chain = MiddlewareChain {
            middlewares: rest,
            controller,
        };
        first.handle(context, req, &mut next_chain)
    } else {
        controller.handle(context, req)
    }
}

pub struct Mediator<Ctx, Req: RequestTrait, Res: ResponseTrait> {
    middlewares: BoxVec<dyn Middleware<Ctx, Req, Res>>,
    controller: Box<dyn Controller<Ctx, Req, Res>>,
}

impl<Ctx, Req: RequestTrait, Res: ResponseTrait> Mediator<Ctx, Req, Res> {
    pub fn new(
        middlewares: BoxVec<dyn Middleware<Ctx, Req, Res>>,
        controller: Box<dyn Controller<Ctx, Req, Res>>,
    ) -> Self {
        Self {
            middlewares,
            controller,
        }
    }

    pub fn with_middleware(mut self, middleware: Box<dyn Middleware<Ctx, Req, Res>>) -> Self {
        self.middlewares.push(middleware);
        self
    }

    pub fn set_controller(&mut self, controller: Box<dyn Controller<Ctx, Req, Res>>) {
        self.controller = controller;
    }
}

impl<Ctx, Req: RequestTrait, Res: ResponseTrait> Controller<Ctx, Req, Res>
    for Mediator<Ctx, Req, Res>
{
    fn handle(&mut self, context: &Ctx, req: &mut Req) -> Res {
        execute_middleware_chain(&mut self.middlewares, self.controller.as_mut(), context, req)
    }
}