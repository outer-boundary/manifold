use std::fmt::Display;

use crate::Error;

pub struct Url {
    pub protocol: UrlProtocol,
    pub host: String,
    pub port: u16,
    pub path: RouteCollection,
}

pub enum UrlProtocol {
    Http,
    Https,
}

impl TryFrom<&str> for UrlProtocol {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "http" => Ok(UrlProtocol::Http),
            "https" => Ok(UrlProtocol::Https),
            _ => Err(format!("Invalid URL protocol: {}", value).into()),
        }
    }
}

impl TryFrom<String> for UrlProtocol {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, self::Error> {
        UrlProtocol::try_from(value.as_str())
    }
}

pub struct RouteCollection {
    pub routes: Vec<String>,
}

impl Display for RouteCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.routes.join("/").as_str())
    }
}

impl From<Vec<String>> for RouteCollection {
    fn from(value: Vec<String>) -> Self {
        RouteCollection { routes: value }
    }
}
