use crate::http::{Method, ServerRequest};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct ParameterPattern {
    pub name: String,
    pub pattern: Regex,
}

#[derive(Debug, Clone)]
pub struct Route {
    pub name: String,
    pub path: String,
    pub patterns: Vec<ParameterPattern>,
    pub accept: Vec<Method>,
    pub handler: String,
    filled: Vec<String>,
}

impl Route {
    pub fn new(
        name: String,
        path: String,
        patterns: Vec<ParameterPattern>,
        accept: Vec<Method>,
        handler: String,
        filled: Vec<String>,
    ) -> Self {
        Self {
            name,
            path,
            patterns,
            accept,
            handler,
            filled,
        }
    }

    pub fn is_allowed(&self, method: Method) -> bool {
        self.accept.contains(&method)
    }

    pub fn matches(&mut self, request: &ServerRequest) -> bool {
        self.filled.clear();
        let mut path = request.configuration.path_info();
        for parameter in &self.patterns {
            path = path.replace(
                &format!("{{{}}}", parameter.name),
                &format!("(?<{}>{})", parameter.name, parameter.pattern.to_string()),
            );
        }
        let pattern = Regex::new(&format!("^{}$", self.path)).unwrap();
        if !pattern.is_match(&path) {
            return false;
        }
        let captures = pattern.captures(&path).unwrap();
        for parameter in self.patterns.iter() {
            self.filled
                .push(captures.name(&parameter.name).unwrap().as_str().to_string());
        }
        true
    }

    pub fn parameters(&self) -> Vec<String> {
        self.filled.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Router {
    pub routes: Vec<Route>,
}

impl Router {
    pub fn new(routes: Vec<Route>) -> Self {
        Self { routes }
    }
    pub fn get(&self, route_name: &str) -> Option<&Route> {
        self.routes.iter().find(|r| r.name == route_name)
    }
    
    pub fn route(&mut self, request: &ServerRequest) -> Option<(&Route, Vec<String>)> {
        for route in &mut self.routes {
            if route.matches(request) {
                return Some((route, route.parameters()));
            }
        }
        None
    }
}