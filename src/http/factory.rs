//! Factories for building HTTP requests and responses.
use crate::http::{
    Headers, Message, MessageTrait, Method, Request, Response, Status, Uri, Version,
};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
/// Helper for constructing [`Request`] instances with preset defaults.
pub struct RequestFactory {
    pub version: Version,
    pub default_headers: Headers,
}

impl RequestFactory {
    /// Create a new factory using the given HTTP version and default headers.
    pub fn new(version: Version, default_headers: Headers) -> Self {
        Self {
            version,
            default_headers,
        }
    }

    /// Shorthand for creating a factory with empty default headers.
    pub fn version(version: Version) -> Self {
        Self::new(version, Headers::new())
    }

    /// Build a [`Request`] with the provided method, target and data.
    pub fn build(&self, method: Method, target: Uri, headers: Headers, body: &str) -> Request {
        Request {
            method,
            target,
            message: Message {
                version: self.version,
                headers: self.default_headers.merge_with(&headers),
                body: body.to_string(),
            },
        }
    }

    /// Build a GET request to `target` using `headers`.
    pub fn get(&self, target: Uri, headers: Headers) -> Request {
        self.build(Method::Get, target, headers, "")
    }

    /// Build a POST request to `target` using `headers` and `body`.
    pub fn post(&self, target: Uri, headers: Headers, body: &str) -> Request {
        self.build(Method::Post, target, headers, body)
    }
}

#[derive(Debug, Clone)]
pub enum Redirection {
    /// The 300 (Multiple Choices) status code indicates that the target resource has more than one representation,
    /// each with its own specific location, and the user or user agent should select one of them.
    /// The Location header field can be used to provide the URI for the preferred choice,
    /// but the available choices are not limited to those in the response.
    ///
    /// The MultipleChoices variant accepts a list of possible URIs (`Vec<Uri>`) and an optional
    /// preferred URI (`Option<Uri>`).
    MultipleChoices(Vec<Uri>, Option<Uri>),
    /// The 301 (Moved Permanently) status code indicates that the target resource has been assigned
    /// a new permanent URI and any future references to this resource ought to use one of the enclosed URIs.
    /// The handler is suggesting that a user agent with link-editing capability can permanently replace
    /// references to the target URI with one of the new references sent by the handler.
    ///
    /// However, this suggestion is usually ignored unless the user agent is actively editing references (e.g., engaged in authoring content),
    /// the connection is secured, and the origin handler is a trusted authority for the content being edited.
    MovedPermanently(Uri),
    /// The 302 (Found) status code indicates that the target resource resides temporarily under
    /// a different URI.
    /// Since the redirection might be altered on occasion, the client ought to continue to use
    /// the target URI for future requests.
    Found(Uri),
    /// The 303 (See Other) status code indicates that the handler is redirecting the user agent to
    /// a different resource, as indicated by a URI in the Location header field, which is intended
    /// to provide an indirect response to the original request.
    ///
    /// A user agent can perform a retrieval request targeting that URI (a GET or HEAD request if using HTTP),
    /// which might also be redirected, and present the eventual result as an answer to the original request.
    ///
    /// Note that the new URI in the Location header field is not considered equivalent to the target URI.
    SeeOther(Uri),
    /// The 304 (Not Modified) status code indicates that a conditional GET or HEAD request has been received
    /// and would have resulted in a 200 (OK) response if it were not for the fact that the condition evaluated to false.
    /// In other words, there is no need for the handler to transfer a representation of the target resource
    /// because the request indicates that the client, which made the request conditional,
    /// already has a valid representation;
    /// the handler is therefore redirecting the client to make use of that stored representation as if
    /// it were the content of a 200 (OK) response.
    NotModified(Uri, Headers),
    /// The 307 (Temporary Redirect) status code indicates that the target resource resides temporarily under
    /// a different URI and the user agent MUST NOT change the request method if it performs an automatic
    /// redirection to that URI. Since the redirection can change over time, the client ought to continue using
    /// the original target URI for future requests.
    TemporaryRedirect(Uri),
    /// The 308 (Permanent Redirect) status code indicates that the target resource has been assigned a new permanent
    /// URI and any future references to this resource ought to use one of the enclosed URIs.
    ///
    /// The handler is suggesting that a user agent with link-editing capability can permanently replace
    /// references to the target URI with one of the new references sent by the handler.
    /// However, this suggestion is usually ignored unless the user agent is actively editing references
    /// (e.g., engaged in authoring content), the connection is secured, and the origin handler is a trusted
    /// authority for the content being edited.
    PermanentRedirect(Uri),
}

impl Redirection {
    pub fn to_status(&self) -> Status {
        match self {
            Redirection::MultipleChoices(_, _) => Status::MultipleChoices,
            Redirection::MovedPermanently(_) => Status::MovedPermanently,
            Redirection::Found(_) => Status::Found,
            Redirection::SeeOther(_) => Status::SeeOther,
            Redirection::NotModified(_, _) => Status::NotModified,
            Redirection::TemporaryRedirect(_) => Status::TemporaryRedirect,
            Redirection::PermanentRedirect(_) => Status::PermanentRedirect,
        }
    }

    pub fn get_uri(&self) -> &Uri {
        match self {
            Redirection::MultipleChoices(uris, pref) => {
                pref.as_ref().unwrap_or_else(|| uris.first().unwrap())
            }
            Redirection::MovedPermanently(uri)
            | Redirection::Found(uri)
            | Redirection::SeeOther(uri)
            | Redirection::NotModified(uri, _)
            | Redirection::TemporaryRedirect(uri)
            | Redirection::PermanentRedirect(uri) => uri,
        }
    }

    pub fn to_headers(&self) -> Headers {
        let mut headers = Headers::new();
        headers.insert("Location", &[self.get_uri().to_string()]);
        if let Redirection::NotModified(_, r_headers) = self {
            for (key, value) in r_headers.iter() {
                headers.insert(key, value);
            }
        }
        if let Redirection::MultipleChoices(uris, _) = self {
            for uri in uris {
                headers.insert("Link", &[format!("href=\"{}\"; rel=alternative", uri)]);
            }
        }
        headers
    }

    pub fn to_pair(&self) -> (Status, Headers) {
        (self.to_status(), self.to_headers())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthenticationScheme {
    Basic,
    Bearer,
    Concealed,
    Digest,
    Dpop,
    Gnap,
    Hoba,
    Mutual,
    Negociate,
    OAuth,
    PrivateToken,
    ScramSha1,
    ScramSha256,
    Vapid,
}

impl Display for AuthenticationScheme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AuthenticationScheme::Basic => "Basic",
                AuthenticationScheme::Bearer => "Bearer",
                AuthenticationScheme::Concealed => "Concealed",
                AuthenticationScheme::Digest => "Digest",
                AuthenticationScheme::Dpop => "DPoP",
                AuthenticationScheme::Gnap => "GNAP",
                AuthenticationScheme::Hoba => "HOBA",
                AuthenticationScheme::Mutual => "Mutual",
                AuthenticationScheme::Negociate => "Negotiate",
                AuthenticationScheme::OAuth => "OAuth",
                AuthenticationScheme::PrivateToken => "PrivateToken",
                AuthenticationScheme::ScramSha1 => "SCRAM-SHA-1",
                AuthenticationScheme::ScramSha256 => "SCRAM-SHA-256",
                AuthenticationScheme::Vapid => "vapid",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WWWAuthenticate {
    pub scheme: AuthenticationScheme,
    pub realm: Option<String>,
    pub charset: Option<String>,
}

impl Display for WWWAuthenticate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}{}",
            self.scheme,
            if let Some(realm) = &self.realm {
                format!("realm={}", realm)
            } else {
                "".to_string()
            },
            if let Some(charset) = &self.charset {
                format!(
                    "{}charset={}",
                    if self.realm.is_some() { ", " } else { "" },
                    charset
                )
            } else {
                "".to_string()
            }
        )
    }
}

#[derive(Debug, Clone)]
/// Helper for constructing [`Response`] instances with preset defaults.
pub struct ResponseFactory {
    pub version: Version,
    pub default_headers: Headers,
}

impl ResponseFactory {
    /// Create a new factory using the given HTTP version and default headers.
    pub fn new(version: Version, default_headers: Headers) -> Self {
        Self {
            version,
            default_headers,
        }
    }

    /// Shorthand for creating a factory with empty default headers.
    pub fn version(version: Version) -> Self {
        Self::new(version, Headers::new())
    }

    /// Construct a [`Response`] with the supplied status and headers.
    pub fn with_status(&self, status: Status, headers: Headers) -> Response {
        Response {
            status,
            message: Message {
                version: self.version,
                headers: self.default_headers.merge_with(&headers),
                body: "".to_string(),
            },
        }
    }
    /// Build a redirection response with `Location` header.
    pub fn redirect(&self, redirection: Redirection) -> Response {
        let mut headers = Headers::new();
        let (status, target) = redirection.to_pair();
        headers.insert("Location", &[target.to_string()]);
        self.with_status(status, headers)
    }
    /// Convenience helper to return a 200 response.
    pub fn ok(&self, headers: Headers, body: String) -> Response {
        self.with_status(Status::OK, headers).with_body(&body)
    }
    /// Generate a 204-like empty response.
    pub fn no_content(&self, headers: Headers) -> Response {
        self.with_status(Status::OK, headers)
    }
    /// Create a 300 Multiple Choices response.
    pub fn multiple_choice(&self, uris: Vec<Uri>, preferred: Option<Uri>) -> Response {
        self.redirect(Redirection::MultipleChoices(uris, preferred))
    }
    /// Create a 301 response pointing permanently to `uri`.
    pub fn moved_permanently(&self, uri: Uri) -> Response {
        self.redirect(Redirection::MovedPermanently(uri))
    }
    /// Create a 302 response pointing to `uri`.
    pub fn found(&self, uri: Uri) -> Response {
        self.redirect(Redirection::Found(uri))
    }
    /// Create a 303 response pointing to `uri`.
    pub fn see_other(&self, uri: Uri) -> Response {
        self.redirect(Redirection::SeeOther(uri))
    }
    /// Create a 304 response using headers from `headers`.
    pub fn not_modified(&self, uri: Uri, headers: Headers) -> Response {
        self.redirect(Redirection::NotModified(uri, headers))
    }
    /// Create a 307 Temporary Redirect to `uri`.
    pub fn temporary_redirect(&self, uri: Uri) -> Response {
        self.redirect(Redirection::TemporaryRedirect(uri))
    }
    /// Create a 308 Permanent Redirect to `uri`.
    pub fn permanent_redirect(&self, uri: Uri) -> Response {
        self.redirect(Redirection::PermanentRedirect(uri))
    }
    /// Return a 401 Unauthorized response with `WWW-Authenticate` header.
    pub fn unauthorized(&self, www_authenticate: WWWAuthenticate, headers: Headers) -> Response {
        let mut headers = headers;
        headers.add("WWW-Authenticate", &www_authenticate.to_string());
        self.with_status(Status::Unauthorized, headers)
    }
    /// Return a 403 Forbidden response.
    pub fn forbidden(&self, headers: Headers) -> Response {
        self.with_status(Status::Forbidden, headers)
    }
    /// Return a 501 Not Implemented response with a body.
    pub fn not_implemented(&self, message: &str) -> Response {
        self.with_status(Status::NotImplemented, Headers::new())
            .with_body(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::http::{Authority, Path, Query};

    #[test]
    fn test_request_factory() {
        let factory = RequestFactory::version(Version::Http1_1);
        let uri = Uri::new("http".into(), Authority::new("host".into(), None, None, None), Path::new("/".into(), None), Query::new(), None);
        let req = factory.get(uri.clone(), Headers::new());
        assert_eq!(req.method, Method::Get);
        let req2 = factory.post(uri.clone(), Headers::new(), "b");
        assert_eq!(req2.method, Method::Post);
    }

    #[test]
    fn test_redirection_and_response_factory() {
        let factory = ResponseFactory::version(Version::Http1_1);
        let target = Uri::new("http".into(), Authority::new("host".into(), None, None, None), Path::new("/".into(), None), Query::new(), None);
        let resp = factory.moved_permanently(target.clone());
        assert_eq!(resp.status, Status::MovedPermanently);
        assert!(resp.message.raw().contains("http://host//"));

        let resp = factory.ok(Headers::new(), "body".to_string());
        assert_eq!(resp.status, Status::OK);
        assert_eq!(resp.body(), "body");

        let www = WWWAuthenticate { scheme: AuthenticationScheme::Basic, realm: Some("r".to_string()), charset: None };
        let resp = factory.unauthorized(www, Headers::new());
        assert_eq!(resp.status, Status::Unauthorized);
    }
}
