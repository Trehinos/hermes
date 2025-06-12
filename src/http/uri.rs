//! Types for parsing and representing URIs.
use crate::concepts::{both_or_none, Parsable};
use crate::http::request::Query;
use nom::bytes::complete::{tag, take_till, take_until};
use nom::bytes::take_while;
use nom::character::anychar;
use nom::combinator::opt;
use nom::multi::fold_many0;
use nom::sequence::preceded;
use nom::IResult;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone)]
/// Represents the resource path and optional extra path info of a URI.
pub struct Path {
    pub resource: String,
    pub path_info: Option<String>,
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.resource,
            self.path_info.as_deref().unwrap_or_default()
        )
    }
}

impl Path {
    /// Create a new [`Path`] with the provided resource and optional path info.
    pub fn new(resource: String, path_info: Option<String>) -> Self {
        Self {
            resource,
            path_info,
        }
    }
}

impl Parsable for Path {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        use nom::Parser;

        let (input, path) = take_till(|c| c == '?' || c == '#')(input)?;
        if !path.contains('/') {
            return Ok((input, Self::new(path.to_string(), None)));
        }
        let (path, starting_slashes) = take_while(|c| c == '/').parse(path)?;

        let parts = path.split('/').collect::<Vec<&str>>();
        let mut resource = "".to_string();
        let mut path_info = None;
        let mut filename = false;
        for part in parts {
            if filename {
                if path_info.is_none() {
                    path_info = Some(format!("/{}", part));
                } else {
                    path_info = Some(format!("{}/{}", path_info.unwrap(), part));
                }
            } else {
                if part.contains('.') {
                    filename = true;
                }
                resource = if resource.is_empty() {
                    part.replace("%20", " ").replace("+", " ")
                } else {
                    format!(
                        "{}/{}",
                        resource,
                        part.replace("%20", " ").replace("+", " ")
                    )
                };
            }
        }
        Ok((
            input,
            Self::new(format!("{}{}", starting_slashes, resource), path_info),
        ))
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
/// User information, host and port part of a URI.
pub struct Authority {
    pub host: String,
    pub user: Option<String>,
    pub password: Option<String>,
    pub port: Option<u16>,
}

impl Authority {
    /// Create a new [`Authority`] instance from its components.
    pub fn new(
        host: String,
        user: Option<String>,
        password: Option<String>,
        port: Option<u16>,
    ) -> Self {
        Self {
            host,
            user,
            password,
            port,
        }
    }
}

impl Authority {
    /// Parse `user:password` information from an authority string.
    pub fn parse_user_info(input: &str) -> IResult<&str, (Option<String>, Option<String>)> {
        let user: Option<String>;
        let mut password: Option<String> = None;
        if input.contains(":") {
            let (p, u) = take_until(":")(input)?;
            let (p, _) = tag(":")(p)?;
            user = Some(u.to_string());
            password = Some(p.to_string());
        } else {
            user = Some(input.to_string());
        }
        Ok(("", (user, password)))
    }

    /// Parse the host and optional port from an authority part.
    pub fn parse_host(input: &str) -> IResult<&str, (String, Option<u16>)> {
        let host: String;
        let mut port: Option<u16> = None;
        if input.contains(":") {
            let (p, h) = take_until(":")(input)?;
            host = h.to_string();
            port = Some(p.parse::<u16>().unwrap_or(80));
        } else {
            host = input.to_string();
        }
        Ok(("", (host, port)))
    }
}

impl Parsable for Authority {
    /// Parse the authority component of a URI.
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let mut input = input;
        let mut authority: &str;
        let mut user: Option<String> = None;
        let mut password: Option<String> = None;
        if input.contains("/") {
            let (i, a) = take_until("/")(input)?;
            authority = a;
            if authority.contains("@") {
                let (host_part, user_info) = take_until("@")(authority)?;
                let (host_part, _) = tag("@")(host_part)?;
                authority = host_part;
                let (_, (u, ps)) = Self::parse_user_info(user_info)?;
                user = u;
                password = ps;
            }
            input = i;
        } else {
            let (i, a) = take_till(|c| c == '?' || c == '#' || c == '&')(input)?;
            input = i;
            authority = a;
        }
        let (_, (host, port)) = Self::parse_host(authority)?;
        Ok((
            input,
            Self {
                host,
                user,
                password,
                port,
            },
        ))
    }
}

#[derive(Debug, Default, Clone)]
/// A fully parsed Uniform Resource Identifier.
pub struct Uri {
    pub scheme: String,
    pub authority: Authority,
    pub path: Path,
    pub query: Query,
    pub fragment: Option<String>,
}

impl Parsable for Uri {
    /// Parse a URI from a string slice.
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        use nom::Parser;

        let mut input = input;
        let mut scheme = Self::SCHEME_NONE;
        let mut authority = Authority::default();
        if input.contains(":") {
            let (i, s) = take_until(":")(input)?;
            scheme = s;
            let (i, _) = tag(":")(i)?;
            input = i;
            if i.starts_with("//") {
                let (i, _) = tag("//")(i)?;
                let (i, a) = Authority::parse(i)?;
                authority = a;
                input = i;
            }
        }
        let (input, path) = Path::parse(input)?;

        let mut fragment = "";
        let mut query = Query::new();
        if input.starts_with('?') {
            let (i, _) = tag("?")(input)?;
            let (f, q) = Query::parse(i)?;
            fragment = f;
            query = q;
        }

        let (_, fragment) = opt(preceded(
            tag("#"),
            fold_many0(anychar, String::new, |mut acc, item| {
                acc.push(item);
                acc
            }),
        ))
        .parse(fragment)?;

        Ok((
            input,
            Self::new(scheme.to_string(), authority, path, query, fragment),
        ))
    }
}

impl Uri {
    pub const SCHEME_NONE: &'static str = "";
    pub const SCHEME_HTTP: &'static str = "http";
    pub const SCHEME_HTTPS: &'static str = "https";
    pub const SCHEME_FTP: &'static str = "ftp";
    pub const SCHEME_SSH: &'static str = "ssh";

    #[allow(clippy::too_many_arguments)]
    /// Construct a new [`Uri`] from its components.
    pub fn new(
        scheme: String,
        authority: Authority,
        path: Path,
        query: Query,
        fragment: Option<String>,
    ) -> Self {
        Self {
            scheme: scheme.clone(),
            authority,
            path,
            query,
            fragment,
        }
    }

    /// Format the authority component back to a string.
    pub fn authority(&self) -> String {
        let user_info = format!(
            "{}{}",
            self.authority
                .user
                .as_ref()
                .map(|u| u.to_string())
                .unwrap_or_default(),
            self.authority
                .password
                .as_ref()
                .map(|p| format!(":{}", p))
                .unwrap_or_default()
        );

        format!(
            "{}{}{}",
            both_or_none(&user_info, "@"),
            self.authority.host,
            self.authority
                .port
                .map(|p| format!(":{}", p))
                .unwrap_or_default()
        )
    }

    /// Return a clone of the [`Path`] component.
    pub fn path(&self) -> Path {
        self.path.clone()
    }
}

impl Display for Uri {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let path = self.path();
        write!(
            f,
            "{}{}{}{}{}",
            both_or_none(&self.scheme, ":"),
            both_or_none(
                "//",
                &if path.resource.is_empty() {
                    self.authority()
                } else {
                    both_or_none(&self.authority(), "/")
                }
            ),
            path,
            both_or_none("?", &self.query.to_string()),
            both_or_none("#", &self.fragment.clone().unwrap_or("".to_string())),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_uri() {
        let uri = "https://host";
        let uri = Uri::parse(uri).unwrap().1;
        assert_eq!(uri.scheme, "https");
        assert_eq!(uri.authority.host, "host");
        assert_eq!(uri.authority.port, None);
        assert_eq!(uri.authority.user, None);
        assert_eq!(uri.authority.password, None);
        assert_eq!(uri.path.resource, "");
        assert_eq!(uri.path.path_info, None);
        assert_eq!(uri.query.to_string(), Query::new().to_string());
        assert_eq!(uri.fragment, None);

        let uri = "http:path";
        let uri = Uri::parse(uri).unwrap().1;
        assert_eq!(uri.scheme, "http");
        assert_eq!(uri.authority.host, "");
        assert_eq!(uri.authority.port, None);
        assert_eq!(uri.authority.user, None);
        assert_eq!(uri.authority.password, None);
        assert_eq!(uri.path.resource, "path");
        assert_eq!(uri.path.path_info, None);
        assert_eq!(uri.query.to_string(), Query::new().to_string());
        assert_eq!(uri.fragment, None);

        let uri = "/path/to/somewhere";
        let uri = Uri::parse(uri).unwrap().1;
        assert_eq!(uri.scheme, Uri::SCHEME_NONE);
        assert_eq!(uri.authority.host, "");
        assert_eq!(uri.authority.port, None);
        assert_eq!(uri.authority.user, None);
        assert_eq!(uri.authority.password, None);
        assert_eq!(uri.path.resource, "/path/to/somewhere");
        assert_eq!(uri.path.path_info, None);
        assert_eq!(uri.query.to_string(), Query::new().to_string());
        assert_eq!(uri.fragment, None);

        let uri = "http://user:pass@host:80//path/resource.ext/path_info?query#fragment";
        let uri = Uri::parse(uri).unwrap().1;
        assert_eq!(uri.scheme, "http");
        assert_eq!(uri.authority.host, "host");
        assert_eq!(uri.authority.port, Some(80));
        assert_eq!(uri.authority.user, Some("user".to_string()));
        assert_eq!(uri.authority.password, Some("pass".to_string()));
        assert_eq!(uri.path.resource, "//path/resource.ext");
        assert_eq!(uri.path.path_info, Some("/path_info".to_string()));
        assert_eq!(uri.query.to_string(), "query=".to_string());
        assert_eq!(uri.fragment, Some("fragment".to_string()));

        let uri = "resource.ext/path_info?query#fragment";
        let uri = Uri::parse(uri).unwrap().1;
        assert_eq!(uri.scheme, Uri::SCHEME_NONE);
        assert_eq!(uri.authority, Authority::default());
        assert_eq!(uri.path.resource, "resource.ext");
        assert_eq!(uri.path.path_info, Some("/path_info".to_string()));
        assert_eq!(uri.query.to_string(), "query=".to_string());
        assert_eq!(uri.fragment, Some("fragment".to_string()));

        let uri = "http://host/path/to/resource.ext/path/info";
        let uri = Uri::parse(uri).unwrap().1;
        assert_eq!(uri.scheme, "http");
        assert_eq!(uri.authority.host, "host");
        assert_eq!(uri.authority.port, None);
        assert_eq!(uri.authority.user, None);
        assert_eq!(uri.authority.password, None);
        assert_eq!(uri.path.resource, "/path/to/resource.ext");
        assert_eq!(uri.path.path_info, Some("/path/info".to_string()));
        assert_eq!(uri.query.to_string(), Query::new().to_string());
        assert_eq!(uri.fragment, None);
    }
    #[test]
    fn test_path_and_authority_display() {
        let path = Path::new("/index.html".to_string(), Some("/info".to_string()));
        assert_eq!(path.to_string(), "/index.html/info");
        let authority = Authority::new("host".to_string(), Some("u".to_string()), Some("p".to_string()), Some(8080));
        assert_eq!(Authority::parse_user_info("user:pass").unwrap().1, (Some("user".to_string()), Some("pass".to_string())));
        assert_eq!(Authority::parse_host("host:80").unwrap().1, ("host".to_string(), Some(80)));
        let uri = Uri::new("http".to_string(), authority.clone(), path.clone(), Query::new(), Some("frag".to_string()));
        assert_eq!(uri.authority(), "u:p@host:8080");
        assert_eq!(uri.path().to_string(), path.to_string());
        assert!(uri.to_string().starts_with("http://u:p@host:8080"));
    }
    }
