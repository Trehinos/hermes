use crate::http::{Method, Middleware, Response, Server, ServerRequest};
use regex::Regex;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Controller {
    pub server: Rc<Server>,
    pub request: ServerRequest,
    pub response: Response,
    pub actions: HashMap<String, Middleware>,
}

impl Controller {
    pub fn to_middleware(self) -> Middleware {
        Middleware::new(
            self.server,
            |server, _| server.response.not_implemented("Controller not implemented"),
            |_| true,
        )
    }
}

pub struct Route {
    pub name: String,
    pub methods: Vec<Method>,
    pub pattern: Regex,
}
