use crate::{
    concepts::Parsable,
    http::{
        AuthenticationScheme, Handler, Headers, ResponseFactory, MiddlewareTrait, Response,
        ServerRequest, Uri, Version, WWWAuthenticate,
    },
    security::{
        authentication::{Authenticator, IdentityPassword, Provider},
        authorization::HasPermissions,
    },
};
use regex::Regex;

pub mod authorization;

pub mod authentication;

pub trait User: IdentityPassword + HasPermissions {
    fn username(&self) -> String;
}

#[derive(Clone)]
pub struct Firewall<'a, U: User> {
    authenticator: &'a dyn Authenticator<U>,
    pub pattern: Regex,
    pub redirect_path: Option<String>,
    pub excluded_paths: Vec<Regex>,
}
impl<'a, U: User> Firewall<'a, U> {
    pub fn protect(
        authenticator: &'a dyn Authenticator<U>,
        pattern: Regex,
    ) -> Self {
        Self {
            authenticator,
            pattern,
            redirect_path: None,
            excluded_paths: vec![],
        }
    }

    pub fn path_is_excluded(&self, path: &str) -> bool {
        self.excluded_paths.iter().any(|r| r.is_match(path))
    }
}

impl<U: User> Handler for Firewall<'_, U> {
    fn check(&self, request: &ServerRequest) -> bool {
        if !self
            .pattern
            .is_match(&request.request().target.path.to_string())
        {
            return false;
        }
        if self.path_is_excluded(&request.request().target.path.to_string()) {
            return false;
        }
        if self.authenticator.is_authenticated() {
            // TODO : Verifier l'autorisation.
            return false;
        }

        true
    }
    fn handle(&mut self, _: &ServerRequest) -> Response {
        let builder = ResponseFactory::version(Version::Http1_1);
        if let Some(r) = &self.redirect_path {
            return builder.temporary_redirect(Uri::parse(r).unwrap().1);
        }
        builder.unauthorized(
            WWWAuthenticate {
                scheme: AuthenticationScheme::Basic,
                realm: None,
                charset: None,
            },
            Headers::new(),
        )
    }
}

impl<U: User> MiddlewareTrait for Firewall<'_, U> {}

pub trait Security<U: User> {
    fn firewalls(&self) -> Vec<Firewall<U>>;
    fn authenticator(&self) -> &dyn Authenticator<U>;
    fn provider(&self) -> &dyn Provider<U>;
    fn current(&self) -> Option<U> {
        self.authenticator().current()
    }
}
