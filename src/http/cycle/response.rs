//! Structures and helpers for HTTP responses.
use crate::concepts::Parsable;
use crate::http::cookie::Cookie;
use crate::http::Headers;
use crate::http::{Message, MessageTrait, Version};
use nom::bytes::complete::take_until;
use nom::character::complete::{digit1, space0, space1};
use nom::combinator::opt;
use nom::IResult;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// HTTP status codes and reasons recognized by the library.
pub enum Status {
    /// 100
    Continue,
    /// 101
    SwitchingProtocols,
    /// 102
    Processing,
    /// 103
    EarlyHints,
    /// 200
    OK,
    /// 201
    Created,
    /// 202
    Accepted,
    /// 203
    NonAuthoritativeInformation,
    /// 204
    NoContent,
    /// 205
    ResetContent,
    /// 206
    PartialContent,
    /// 207
    MultiStatus,
    /// 208
    AlreadyReported,
    /// 226
    IMUsed,
    /// 300
    MultipleChoices,
    /// 301
    MovedPermanently,
    /// 302
    Found,
    /// 303
    SeeOther,
    /// 304
    NotModified,
    /// 305
    ///
    /// Deprecated, see: <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status#305_use_proxy>
    UseProxy,
    /// 307
    TemporaryRedirect,
    /// 308
    PermanentRedirect,
    /// 400
    BadRequest,
    /// 401
    Unauthorized,
    /// 402
    PaymentRequired,
    /// 403
    Forbidden,
    /// 404
    NotFound,
    /// 405
    MethodNotAllowed,
    /// 406
    NotAcceptable,
    /// 407
    ProxyAuthenticationRequired,
    /// 408
    RequestTimeout,
    /// 409
    Conflict,
    /// 410
    Gone,
    /// 411
    LengthRequired,
    /// 412
    PreconditionFailed,
    /// 413
    RequestEntityTooLarge,
    /// 414
    RequestURITooLong,
    /// 415
    UnsupportedMediaType,
    /// 416
    RequestedRangeNotSatisfiable,
    /// 417
    ExpectationFailed,
    /// 418
    ImATeapot,
    /// 421
    MisdirectedRequest,
    /// 422
    UnprocessableEntity,
    /// 423
    Locked,
    /// 424
    FailedDependency,
    /// 425
    TooEarly,
    /// 426
    UpgradeRequired,
    /// 428
    PreconditionRequired,
    /// 429
    TooManyRequests,
    /// 431
    RequestHeaderFieldsTooLarge,
    /// 451
    UnavailableForLegalReasons,
    /// 500
    InternalServerError,
    /// 501
    NotImplemented,
    /// 502
    BadGateway,
    /// 503
    ServiceUnavailable,
    /// 504
    GatewayTimeout,
    /// 505
    HTTPVersionNotSupported,
    /// 506
    VariantAlsoNegotiates,
    /// 507
    InsufficientStorage,
    /// 508
    LoopDetected,
    /// 510
    /// Deprecated, see: <https://datatracker.ietf.org/doc/status-change-http-experiments-to-historic>
    NotExtended,
    /// 511
    NetworkAuthenticationRequired,
    /// Custom status code with an arbitrary reason phrase.
    Custom(u16, String),
}

impl Status {
    /// Return the numeric HTTP status code.
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
            Self::TooEarly => 425,
            Self::UpgradeRequired => 426,
            Self::PreconditionRequired => 428,
            Self::TooManyRequests => 429,
            Self::RequestHeaderFieldsTooLarge => 431,
            Self::UnavailableForLegalReasons => 451,
            Self::InternalServerError => 500,
            Self::NotImplemented => 501,
            Self::BadGateway => 502,
            Self::ServiceUnavailable => 503,
            Self::GatewayTimeout => 504,
            Self::HTTPVersionNotSupported => 505,
            Self::VariantAlsoNegotiates => 506,
            Self::InsufficientStorage => 507,
            Self::LoopDetected => 508,
            Self::NotExtended => 510,
            Self::NetworkAuthenticationRequired => 511,
            Self::Custom(code, _) => *code,
        }
    }

    /// Convert a numeric code to a [`Status`] variant.
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
            425 => Self::TooEarly,
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
            510 => Self::NotExtended,
            511 => Self::NetworkAuthenticationRequired,
            code => Self::Custom(code, String::new()),
        }
    }

    /// Convert a reason phrase to a [`Status`] variant.
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
            "Too Early" => Self::TooEarly,
            "Upgrade Required" => Self::UpgradeRequired,
            "Precondition Required" => Self::PreconditionRequired,
            "Too Many Requests" => Self::TooManyRequests,
            "Request Header Fields Too Large" => Self::RequestHeaderFieldsTooLarge,
            "Unavailable For Legal Reasons" => Self::UnavailableForLegalReasons,
            "Internal Server Error" => Self::InternalServerError,
            "Not Implemented" => Self::NotImplemented,
            "Bad Gateway" => Self::BadGateway,
            "Service Unavailable" => Self::ServiceUnavailable,
            "Gateway Timeout" => Self::GatewayTimeout,
            "HTTP Version Not Supported" => Self::HTTPVersionNotSupported,
            "Variant Also Negotiates" => Self::VariantAlsoNegotiates,
            "Insufficient Storage" => Self::InsufficientStorage,
            "Loop Detected" => Self::LoopDetected,
            "Not Extended" => Self::NotExtended,
            "Network Authentication Required" => Self::NetworkAuthenticationRequired,
            r => Self::Custom(0, r.to_string()),
        }
    }

    /// Return the canonical reason phrase for this status.
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
            Self::TooEarly => "Too Early",
            Self::UpgradeRequired => "Upgrade Required",
            Self::PreconditionRequired => "Precondition Required",
            Self::TooManyRequests => "Too Many Requests",
            Self::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            Self::UnavailableForLegalReasons => "Unavailable For Legal Reasons",
            Self::InternalServerError => "Internal Server Error",
            Self::NotImplemented => "Not Implemented",
            Self::BadGateway => "Bad Gateway",
            Self::ServiceUnavailable => "Service Unavailable",
            Self::GatewayTimeout => "Gateway Timeout",
            Self::HTTPVersionNotSupported => "HTTP Version Not Supported",
            Self::VariantAlsoNegotiates => "Variant Also Negotiates",
            Self::InsufficientStorage => "Insufficient Storage",
            Self::LoopDetected => "Loop Detected",
            Self::NotExtended => "Not Extended",
            Self::NetworkAuthenticationRequired => "Network Authentication Required",
            Self::Custom(_, reason) => reason,
        }
    }

    /// Check if the status code is within the informational range (1xx).
    pub fn is_informational(&self) -> bool {
        (100..=199).contains(&self.to_code())
    }

    /// Check if the status represents success (2xx).
    pub fn is_successful(&self) -> bool {
        (200..=299).contains(&self.to_code())
    }

    /// Check if the status is a redirection code (3xx).
    pub fn is_redirection(&self) -> bool {
        (300..=399).contains(&self.to_code())
    }

    /// Check if the status is a client error (4xx).
    pub fn is_client_error(&self) -> bool {
        (400..=499).contains(&self.to_code())
    }

    /// Check if the status is a server error (5xx).
    pub fn is_server_error(&self) -> bool {
        (500..=599).contains(&self.to_code())
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
        Self: Sized,
    {
        use nom::Parser;

        let (input, code) = digit1(input)?;
        let code = code.parse::<u16>().map_err(|_| {
            nom::Err::Failure(nom::error::Error::new(code, nom::error::ErrorKind::Fail))
        })?;
        let from_code = Status::from_code(code);
        let mut reason = None;
        let (mut input, _) = space0(input)?;
        if input.contains("\r\n") {
            let (i, r) = opt(take_until("\r\n")).parse(input)?;
            input = i;
            reason = r;
        } else if !input.is_empty() {
            reason = Some(input);
            input = "";
        }

        if matches!(from_code, Status::Custom(_, ref r) if r.is_empty()) {
            return Ok((
                input,
                Status::Custom(code, reason.unwrap_or_default().to_string()),
            ));
        }

        if let Some(reason_phrase) = reason {
            if from_code.to_reason() != reason_phrase {
                return Ok((input, Status::Custom(code, reason_phrase.to_string())));
            }
        }

        Ok((input, from_code))
    }
}

/// Behaviour shared by HTTP response types.
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
/// Parsed HTTP response message.
///
/// The [`Response`] type wraps a [`Status`] and a [`Message`] carrying headers
/// and body.  It implements [`ResponseTrait`] allowing high level access to the
/// status code and helper methods for working with headers or the body.
///
/// # Examples
/// ```
/// use hermes::http::{Headers, ResponseFactory, Version, Status, ResponseTrait};
///
/// let factory = ResponseFactory::version(Version::Http1_1);
/// let resp = factory.with_status(Status::OK, Headers::new());
/// assert!(resp.to_string().starts_with("HTTP/1.1 200"));
/// ```
pub struct Response {
    pub status: Status,
    pub message: Message,
}

impl Parsable for Response {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let (input, version) = Version::parse(input)?;
        let (input, _) = space1(input)?;
        let (input, status) = Status::parse(input)?;
        let (_, mut message) = Message::parse(input)?;
        message = message.with_protocol_version(version);
        Ok((input, Self { status, message }))
    }
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
    fn headers(&self) -> &Headers {
        self.message.headers()
    }

    fn headers_mut(&mut self) -> &mut Headers {
        self.message.headers_mut()
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
        self.status.clone()
    }
    fn with_status(self, status: Status) -> Self {
        Self {
            status,
            message: self.message,
        }
    }
}

impl Response {
    /// Return a new response with an additional `Set-Cookie` header.
    pub fn with_cookie(self, cookie: Cookie) -> Self {
        self.with_added_header("Set-Cookie", &[format!("{}={}", cookie.name, cookie.value)])
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
        write!(f, "{}", self.message.raw())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::ResponseFactory;

    #[test]
    fn test_status_helpers() {
        let ok = Status::from_code(200);
        assert_eq!(ok.to_code(), 200);
        assert_eq!(Status::from_reason(ok.to_reason()), ok);
        assert!(ok.is_successful());
        assert!(!ok.is_client_error());
    }

    #[test]
    fn test_status_parse() {
        let (rest, st) = Status::parse("200 OK\r\n").unwrap();
        assert_eq!(rest, "\r\n");
        assert_eq!(st, Status::OK);
    }

    #[test]
    fn test_response_parse_and_methods() {
        let input = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<body></body>";
        let (_, resp) = Response::parse(input).unwrap();
        assert_eq!(resp.code(), 200);
        assert_eq!(resp.reason(), "OK");
        let resp = resp.with_status(Status::NotFound);
        assert_eq!(resp.status(), Status::NotFound);
        assert!(resp.to_string().starts_with("HTTP/1.1 404"));
    }

    #[test]
    fn test_custom_status_round_trip() {
        let (rest, status) = Status::parse("600 My Status\r\n").unwrap();
        assert_eq!(rest, "\r\n");
        assert_eq!(status.to_code(), 600);
        assert_eq!(status.to_reason(), "My Status");
        assert_eq!(status, Status::Custom(600, "My Status".to_string()));

        let line = format!("{} {}\r\n", status.to_code(), status);
        let (rest2, parsed) = Status::parse(&line).unwrap();
        assert_eq!(rest2, "\r\n");
        assert_eq!(parsed, status);
    }

    #[test]
    fn test_response_cookie_helper() {
        let factory = ResponseFactory::version(Version::Http1_1);
        let resp = factory
            .with_status(Status::OK, Headers::new())
            .with_cookie(Cookie::new("a", "1"));
        assert_eq!(resp.get_header_line("Set-Cookie"), Some("a=1".to_string()));
    }
}
