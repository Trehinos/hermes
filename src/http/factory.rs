use crate::http::{
    Headers, Message, MessageTrait, Method, Request, Response, Status, Uri, Version,
};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub struct HttpRequest {
    version: Version,
}

impl HttpRequest {
    pub fn new(version: Version) -> Self {
        Self { version }
    }

    pub fn build(&self, method: Method, target: Uri, headers: Headers, body: &str) -> Request {
        Request {
            method,
            target,
            message: Message {
                version: self.version,
                headers,
                body: body.to_string(),
            },
        }
    }

    pub fn get(&self, target: Uri, headers: Headers) -> Request {
        self.build(Method::Get, target, headers, "")
    }

    pub fn post(&self, target: Uri, headers: Headers, body: &str) -> Request {
        self.build(Method::Post, target, headers, body)
    }
}

#[derive(Debug, Clone)]
pub enum Redirection {
    /// The 301 (Moved Permanently) status code indicates that the target resource has been assigned
    /// a new permanent URI and any future references to this resource ought to use one of the enclosed URIs.
    /// The server is suggesting that a user agent with link-editing capability can permanently replace
    /// references to the target URI with one of the new references sent by the server.
    ///
    /// However, this suggestion is usually ignored unless the user agent is actively editing references (e.g., engaged in authoring content),
    /// the connection is secured, and the origin server is a trusted authority for the content being edited.
    MovedPermanently(Uri),
    /// The 302 (Found) status code indicates that the target resource resides temporarily under
    /// a different URI.
    /// Since the redirection might be altered on occasion, the client ought to continue to use
    /// the target URI for future requests.
    Found(Uri),
    /// The 303 (See Other) status code indicates that the server is redirecting the user agent to
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
    /// In other words, there is no need for the server to transfer a representation of the target resource
    /// because the request indicates that the client, which made the request conditional,
    /// already has a valid representation;
    /// the server is therefore redirecting the client to make use of that stored representation as if
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
    /// The server is suggesting that a user agent with link-editing capability can permanently replace
    /// references to the target URI with one of the new references sent by the server.
    /// However, this suggestion is usually ignored unless the user agent is actively editing references
    /// (e.g., engaged in authoring content), the connection is secured, and the origin server is a trusted
    /// authority for the content being edited.
    PermanentRedirect(Uri),
}

impl Redirection {
    pub fn to_status(&self) -> Status {
        match self {
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

#[derive(Debug, Clone, Copy)]
pub struct HttpResponse {
    version: Version,
}

impl HttpResponse {
    pub fn new(version: Version) -> Self {
        Self { version }
    }
    pub fn with_status(&self, status: Status, headers: Headers) -> Response {
        Response {
            status,
            message: Message {
                version: self.version,
                headers,
                body: "".to_string(),
            },
        }
    }
    pub fn redirect(&self, redirection: Redirection) -> Response {
        let mut headers = Headers::new();
        let (status, target) = redirection.to_pair();
        headers.insert("Location", &[target.to_string()]);
        self.with_status(status, headers)
    }
    pub fn ok(&self, headers: Headers, body: String) -> Response {
        self.with_status(Status::OK, headers).with_body(&body)
    }
    pub fn no_content(&self, headers: Headers) -> Response {
        self.with_status(Status::OK, headers)
    }
    pub fn moved_permanently(&self, uri: Uri) -> Response {
        self.redirect(Redirection::MovedPermanently(uri))
    }
    pub fn found(&self, uri: Uri) -> Response {
        self.redirect(Redirection::Found(uri))
    }
    pub fn see_other(&self, uri: Uri) -> Response {
        self.redirect(Redirection::SeeOther(uri))
    }
    pub fn not_modified(&self, uri: Uri, headers: Headers) -> Response {
        self.redirect(Redirection::NotModified(uri, headers))
    }
    pub fn temporary_redirect(&self, uri: Uri) -> Response {
        self.redirect(Redirection::TemporaryRedirect(uri))
    }
    pub fn permanent_redirect(&self, uri: Uri) -> Response {
        self.redirect(Redirection::PermanentRedirect(uri))
    }
    pub fn unauthorized(&self, www_authenticate: WWWAuthenticate, headers: Headers) -> Response {
        let mut headers = headers;
        headers.add("WWW-Authenticate", &www_authenticate.to_string());
        self.with_status(Status::Unauthorized, headers)
    }
    pub fn forbidden(&self, headers: Headers) -> Response {
        self.with_status(Status::Forbidden, headers)
    }
}
