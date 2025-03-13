use crate::concepts::Parsable;
use crate::http::message::Headers;
use crate::http::{Message, MessageTrait, Version};
use nom::bytes::complete::take_until;
use nom::character::complete::{digit1, space0, space1};
use nom::combinator::opt;
use nom::IResult;
use std::fmt::{Display, Formatter};

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
        Self: Sized,
    {
        use nom::Parser;

        let (input, code) = digit1(input)?;
        let code = code.parse::<u16>().unwrap();
        let mut reason = None;
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

#[cfg(test)]
#[test]
fn test_response_parse() {
    let input = "HTTP/1.1 200 OK\r\n\
        Content-Type: text/html\r\n\
        \r\n\
        <html>...</html>";
    let (_, response) = Response::parse(input).unwrap();
    assert_eq!(response.status, Status::OK);
    assert_eq!(
        response.message.headers().get("Content-Type"),
        Some(&vec!["text/html".to_string()])
    );
    assert_eq!(response.message.body(), "<html>...</html>");
}
