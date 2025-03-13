use crate::http::message::Headers;
use crate::http::uri::Uri;
use crate::http::{Message, MessageTrait, Version};
use std::fmt::{Display, Formatter};
use nom::bytes::complete::take_until;
use nom::character::complete::{digit1, space0};
use nom::combinator::opt;
use nom::IResult;
use crate::concepts::Parsable;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Status {
    Continue,
    SwitchingProtocols,
    Processing,
    EarlyHints,
    OK,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    IMUsed,
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    TemporaryRedirect,
    PermanentRedirect,
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    RequestEntityTooLarge,
    RequestURITooLong,
    UnsupportedMediaType,
    RequestedRangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,
    UnrecoverableError,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HTTPVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    BandwidthLimitExceeded,
    NotExtended,
    NetworkAuthenticationRequired,
}

impl Status {
    pub fn to_code(&self) -> u16 {
        match self {
            Self::Continue => 100,
            Self::SwitchingProtocols => 101,
            Self::Processing => 102,
            Self::EarlyHints => 103,
            Self::OK => 200,
            Self::Created => 201,
            Self::Accepted => 202,
            Self::NonAuthoritativeInformation => 203,
            Self::NoContent => 204,
            Self::ResetContent => 205,
            Self::PartialContent => 206,
            Self::MultiStatus => 207,
            Self::AlreadyReported => 208,
            Self::IMUsed => 226,
            Self::MultipleChoices => 300,
            Self::MovedPermanently => 301,
            Self::Found => 302,
            Self::SeeOther => 303,
            Self::NotModified => 304,
            Self::UseProxy => 305,
            Self::TemporaryRedirect => 307,
            Self::PermanentRedirect => 308,
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::PaymentRequired => 402,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::MethodNotAllowed => 405,
            Self::NotAcceptable => 406,
            Self::ProxyAuthenticationRequired => 407,
            Self::RequestTimeout => 408,
            Self::Conflict => 409,
            Self::Gone => 410,
            Self::LengthRequired => 411,
            Self::PreconditionFailed => 412,
            Self::RequestEntityTooLarge => 413,
            Self::RequestURITooLong => 414,
            Self::UnsupportedMediaType => 415,
            Self::RequestedRangeNotSatisfiable => 416,
            Self::ExpectationFailed => 417,
            Self::ImATeapot => 418,
            Self::MisdirectedRequest => 421,
            Self::UnprocessableEntity => 422,
            Self::Locked => 423,
            Self::FailedDependency => 424,
            Self::UpgradeRequired => 426,
            Self::PreconditionRequired => 428,
            Self::TooManyRequests => 429,
            Self::RequestHeaderFieldsTooLarge => 431,
            Self::UnavailableForLegalReasons => 451,
            Self::UnrecoverableError => 500,
            Self::InternalServerError => 500,
            Self::NotImplemented => 501,
            Self::BadGateway => 502,
            Self::ServiceUnavailable => 503,
            Self::GatewayTimeout => 504,
            Self::HTTPVersionNotSupported => 505,
            Self::VariantAlsoNegotiates => 506,
            Self::InsufficientStorage => 507,
            Self::LoopDetected => 508,
            Self::BandwidthLimitExceeded => 509,
            Self::NotExtended => 510,
            Self::NetworkAuthenticationRequired => 511,
        }
    }

    pub fn from_code(code: u16) -> Self {
        match code {
            100 => Self::Continue,
            101 => Self::SwitchingProtocols,
            102 => Self::Processing,
            103 => Self::EarlyHints,
            200 => Self::OK,
            201 => Self::Created,
            202 => Self::Accepted,
            203 => Self::NonAuthoritativeInformation,
            204 => Self::NoContent,
            205 => Self::ResetContent,
            206 => Self::PartialContent,
            207 => Self::MultiStatus,
            208 => Self::AlreadyReported,
            226 => Self::IMUsed,
            300 => Self::MultipleChoices,
            301 => Self::MovedPermanently,
            302 => Self::Found,
            303 => Self::SeeOther,
            304 => Self::NotModified,
            305 => Self::UseProxy,
            307 => Self::TemporaryRedirect,
            308 => Self::PermanentRedirect,
            400 => Self::BadRequest,
            401 => Self::Unauthorized,
            402 => Self::PaymentRequired,
            403 => Self::Forbidden,
            404 => Self::NotFound,
            405 => Self::MethodNotAllowed,
            406 => Self::NotAcceptable,
            407 => Self::ProxyAuthenticationRequired,
            408 => Self::RequestTimeout,
            409 => Self::Conflict,
            410 => Self::Gone,
            411 => Self::LengthRequired,
            412 => Self::PreconditionFailed,
            413 => Self::RequestEntityTooLarge,
            414 => Self::RequestURITooLong,
            415 => Self::UnsupportedMediaType,
            416 => Self::RequestedRangeNotSatisfiable,
            417 => Self::ExpectationFailed,
            418 => Self::ImATeapot,
            421 => Self::MisdirectedRequest,
            422 => Self::UnprocessableEntity,
            423 => Self::Locked,
            424 => Self::FailedDependency,
            426 => Self::UpgradeRequired,
            428 => Self::PreconditionRequired,
            429 => Self::TooManyRequests,
            431 => Self::RequestHeaderFieldsTooLarge,
            451 => Self::UnavailableForLegalReasons,
            500 => Self::InternalServerError,
            501 => Self::NotImplemented,
            502 => Self::BadGateway,
            503 => Self::ServiceUnavailable,
            504 => Self::GatewayTimeout,
            505 => Self::HTTPVersionNotSupported,
            506 => Self::VariantAlsoNegotiates,
            507 => Self::InsufficientStorage,
            508 => Self::LoopDetected,
            509 => Self::BandwidthLimitExceeded,
            510 => Self::NotExtended,
            511 => Self::NetworkAuthenticationRequired,
            _ => panic!("Invalid status code"),
        }
    }

    pub fn from_reason(reason: &str) -> Self {
        match reason {
            "Continue" => Self::Continue,
            "Switching Protocols" => Self::SwitchingProtocols,
            "Processing" => Self::Processing,
            "Early Hints" => Self::EarlyHints,
            "OK" => Self::OK,
            "Created" => Self::Created,
            "Accepted" => Self::Accepted,
            "Non-Authoritative Information" => Self::NonAuthoritativeInformation,
            "No Content" => Self::NoContent,
            "Reset Content" => Self::ResetContent,
            "Partial Content" => Self::PartialContent,
            "Multi-Status" => Self::MultiStatus,
            "Already Reported" => Self::AlreadyReported,
            "IM Used" => Self::IMUsed,
            "Multiple Choices" => Self::MultipleChoices,
            "Moved Permanently" => Self::MovedPermanently,
            "Found" => Self::Found,
            "See Other" => Self::SeeOther,
            "Not Modified" => Self::NotModified,
            "Use Proxy" => Self::UseProxy,
            "Temporary Redirect" => Self::TemporaryRedirect,
            "Permanent Redirect" => Self::PermanentRedirect,
            "Bad Request" => Self::BadRequest,
            "Unauthorized" => Self::Unauthorized,
            "Payment Required" => Self::PaymentRequired,
            "Forbidden" => Self::Forbidden,
            "Not Found" => Self::NotFound,
            "Method Not Allowed" => Self::MethodNotAllowed,
            "Not Acceptable" => Self::NotAcceptable,
            "Proxy Authentication Required" => Self::ProxyAuthenticationRequired,
            "Request Timeout" => Self::RequestTimeout,
            "Conflict" => Self::Conflict,
            "Gone" => Self::Gone,
            "Length Required" => Self::LengthRequired,
            "Precondition Failed" => Self::PreconditionFailed,
            "Request Entity Too Large" => Self::RequestEntityTooLarge,
            "Request URI Too Long" => Self::RequestURITooLong,
            "Unsupported Media Type" => Self::UnsupportedMediaType,
            "Requested Range Not Satisfiable" => Self::RequestedRangeNotSatisfiable,
            "Expectation Failed" => Self::ExpectationFailed,
            "I'm a teapot" => Self::ImATeapot,
            "Misdirected Request" => Self::MisdirectedRequest,
            "Unprocessable Entity" => Self::UnprocessableEntity,
            "Locked" => Self::Locked,
            "Failed Dependency" => Self::FailedDependency,
            "Upgrade Required" => Self::UpgradeRequired,
            "Precondition Required" => Self::PreconditionRequired,
            "Too Many Requests" => Self::TooManyRequests,
            "Request Header Fields Too Large" => Self::RequestHeaderFieldsTooLarge,
            "Unavailable For Legal Reasons" => Self::UnavailableForLegalReasons,
            "Unrecoverable Error" => Self::UnrecoverableError,
            "Internal Server Error" => Self::InternalServerError,
            "Not Implemented" => Self::NotImplemented,
            "Bad Gateway" => Self::BadGateway,
            "Service Unavailable" => Self::ServiceUnavailable,
            "Gateway Timeout" => Self::GatewayTimeout,
            "HTTP Version Not Supported" => Self::HTTPVersionNotSupported,
            "Variant Also Negotiates" => Self::VariantAlsoNegotiates,
            "Insufficient Storage" => Self::InsufficientStorage,
            "Loop Detected" => Self::LoopDetected,
            "Bandwidth Limit Exceeded" => Self::BandwidthLimitExceeded,
            "Not Extended" => Self::NotExtended,
            "Network Authentication Required" => Self::NetworkAuthenticationRequired,
            r => panic!("Invalid status reason '{}'", r),
        }
    }

    pub fn to_reason(&self) -> &str {
        match self {
            Self::Continue => "Continue",
            Self::SwitchingProtocols => "Switching Protocols",
            Self::Processing => "Processing",
            Self::EarlyHints => "Early Hints",
            Self::OK => "OK",
            Self::Created => "Created",
            Self::Accepted => "Accepted",
            Self::NonAuthoritativeInformation => "Non-Authoritative Information",
            Self::NoContent => "No Content",
            Self::ResetContent => "Reset Content",
            Self::PartialContent => "Partial Content",
            Self::MultiStatus => "Multi-Status",
            Self::AlreadyReported => "Already Reported",
            Self::IMUsed => "IM Used",
            Self::MultipleChoices => "Multiple Choices",
            Self::MovedPermanently => "Moved Permanently",
            Self::Found => "Found",
            Self::SeeOther => "See Other",
            Self::NotModified => "Not Modified",
            Self::UseProxy => "Use Proxy",
            Self::TemporaryRedirect => "Temporary Redirect",
            Self::PermanentRedirect => "Permanent Redirect",
            Self::BadRequest => "Bad Request",
            Self::Unauthorized => "Unauthorized",
            Self::PaymentRequired => "Payment Required",
            Self::Forbidden => "Forbidden",
            Self::NotFound => "Not Found",
            Self::MethodNotAllowed => "Method Not Allowed",
            Self::NotAcceptable => "Not Acceptable",
            Self::ProxyAuthenticationRequired => "Proxy Authentication Required",
            Self::RequestTimeout => "Request Timeout",
            Self::Conflict => "Conflict",
            Self::Gone => "Gone",
            Self::LengthRequired => "Length Required",
            Self::PreconditionFailed => "Precondition Failed",
            Self::RequestEntityTooLarge => "Request Entity Too Large",
            Self::RequestURITooLong => "Request URI Too Long",
            Self::UnsupportedMediaType => "Unsupported Media Type",
            Self::RequestedRangeNotSatisfiable => "Requested Range Not Satisfiable",
            Self::ExpectationFailed => "Expectation Failed",
            Self::ImATeapot => "I'm a teapot",
            Self::MisdirectedRequest => "Misdirected Request",
            Self::UnprocessableEntity => "Unprocessable Entity",
            Self::Locked => "Locked",
            Self::FailedDependency => "Failed Dependency",
            Self::UpgradeRequired => "Upgrade Required",
            Self::PreconditionRequired => "Precondition Required",
            Self::TooManyRequests => "Too Many Requests",
            Self::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            Self::UnavailableForLegalReasons => "Unavailable For Legal Reasons",
            Self::UnrecoverableError => "Unrecoverable Error",
            Self::InternalServerError => "Internal Server Error",
            Self::NotImplemented => "Not Implemented",
            Self::BadGateway => "Bad Gateway",
            Self::ServiceUnavailable => "Service Unavailable",
            Self::GatewayTimeout => "Gateway Timeout",
            Self::HTTPVersionNotSupported => "HTTP Version Not Supported",
            Self::VariantAlsoNegotiates => "Variant Also Negotiates",
            Self::InsufficientStorage => "Insufficient Storage",
            Self::LoopDetected => "Loop Detected",
            Self::BandwidthLimitExceeded => "Bandwidth Limit Exceeded",
            Self::NotExtended => "Not Extended",
            Self::NetworkAuthenticationRequired => "Network Authentication Required",
        }
    }

    pub fn is_informational(&self) -> bool {
        matches!(
            self,
            Self::Continue | Self::SwitchingProtocols | Self::Processing | Self::EarlyHints
        )
    }

    pub fn is_successful(&self) -> bool {
        matches!(
            self,
            Self::Continue
                | Self::SwitchingProtocols
                | Self::Processing
                | Self::OK
                | Self::Created
                | Self::Accepted
                | Self::NonAuthoritativeInformation
                | Self::NoContent
                | Self::ResetContent
                | Self::PartialContent
                | Self::MultiStatus
                | Self::AlreadyReported
                | Self::IMUsed
        )
    }

    pub fn is_redirection(&self) -> bool {
        matches!(
            self,
            Self::MultipleChoices
                | Self::MovedPermanently
                | Self::Found
                | Self::SeeOther
                | Self::NotModified
                | Self::UseProxy
                | Self::TemporaryRedirect
        )
    }

    pub fn is_client_error(&self) -> bool {
        matches!(
            self,
            Self::BadRequest
                | Self::Unauthorized
                | Self::PaymentRequired
                | Self::Forbidden
                | Self::NotFound
                | Self::MethodNotAllowed
                | Self::NotAcceptable
                | Self::ProxyAuthenticationRequired
                | Self::RequestTimeout
                | Self::Conflict
                | Self::Gone
                | Self::LengthRequired
                | Self::PreconditionFailed
                | Self::RequestEntityTooLarge
                | Self::RequestURITooLong
                | Self::UnsupportedMediaType
                | Self::RequestedRangeNotSatisfiable
                | Self::ExpectationFailed
                | Self::ImATeapot
                | Self::MisdirectedRequest
                | Self::UnprocessableEntity
                | Self::Locked
                | Self::FailedDependency
                | Self::UpgradeRequired
                | Self::PreconditionRequired
                | Self::TooManyRequests
                | Self::RequestHeaderFieldsTooLarge
                | Self::UnavailableForLegalReasons
        )
    }

    pub fn is_server_error(&self) -> bool {
        matches!(
            self,
            Self::InternalServerError
                | Self::NotImplemented
                | Self::BadGateway
                | Self::ServiceUnavailable
                | Self::GatewayTimeout
                | Self::HTTPVersionNotSupported
                | Self::VariantAlsoNegotiates
                | Self::InsufficientStorage
                | Self::LoopDetected
                | Self::BandwidthLimitExceeded
                | Self::NotExtended
                | Self::NetworkAuthenticationRequired
                | Self::UnrecoverableError
        )
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_reason())
    }
}

impl Parsable for Status {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized
    {
        use nom::Parser;

        let (input, code) = digit1(input)?;
        let code = code.parse::<u16>().unwrap();
        let mut reason = None ;
        let from_code = Status::from_code(code);
        let (mut input, _) = space0(input)?;
        if input.contains("\r\n") {
            let (i, r) = opt(take_until("\r\n")).parse(input)?;
            input = i;
            reason = r;
        } else if !input.is_empty() {
            reason = Some(input);
            input = "";
        }
        if let Some(reason) = reason {
            let reason = Status::from_reason(reason);
            if from_code != reason {
                panic!("Invalid status code: {}, for reason: {}", code, reason);
            }
        }

        Ok((input, Self::from_code(code)))
    }
}

#[cfg(test)]
#[test]
fn test_status_parse() {
    let input = "200";
    let (input, status) = Status::parse(input).unwrap();
    assert_eq!(input, "");
    assert_eq!(status, Status::OK);

    let input = "200 OK";
    let (input, status) = Status::parse(input).unwrap();
    assert_eq!(input, "");
    assert_eq!(status, Status::OK);

    let input = "200 OK\r\n...";
    let (input, status) = Status::parse(input).unwrap();
    assert_eq!(input, "\r\n...");
    assert_eq!(status, Status::OK);
}

pub trait ResponseTrait: MessageTrait {
    fn status(&self) -> Status;
    fn code(&self) -> u16 {
        self.status().to_code()
    }
    fn reason(&self) -> String {
        self.status().to_string()
    }
    fn with_status(self, status: Status) -> Self
    where
        Self: Sized;
}

#[derive(Debug, Clone)]
pub struct Response {
    pub status: Status,
    pub message: Message,
}

impl MessageTrait for Response {
    fn protocol_version(&self) -> Version {
        self.message.protocol_version()
    }
    fn with_protocol_version(self, version: Version) -> Self {
        Self {
            status: self.status,
            message: self.message.with_protocol_version(version),
        }
    }
    fn headers(&self) -> Headers {
        self.message.headers()
    }
    fn has_header(&self, key: &str) -> bool {
        self.message.has_header(key)
    }
    fn with_headers(self, headers: Headers) -> Self {
        Self {
            status: self.status,
            message: self.message.with_headers(headers),
        }
    }

    fn with_added_header(self, key: &str, value: &[String]) -> Self
    where
        Self: Sized,
    {
        Self {
            status: self.status,
            message: self.message.with_added_header(key, value),
        }
    }

    fn without_header(self, key: &str) -> Self
    where
        Self: Sized,
    {
        Self {
            status: self.status,
            message: self.message.without_header(key),
        }
    }

    fn body(&self) -> String {
        self.message.body()
    }

    fn with_body(self, body: &str) -> Self
    where
        Self: Sized,
    {
        Self {
            status: self.status,
            message: self.message.with_body(body),
        }
    }
}

impl ResponseTrait for Response {
    fn status(&self) -> Status {
        self.status
    }
    fn with_status(self, status: Status) -> Self {
        Self {
            status,
            message: self.message,
        }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}\r\n",
            self.message.version,
            self.code(),
            self.reason()
        )?;
        write!(f, "{}", self.message)
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
