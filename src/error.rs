use std::fmt::{self};

/// The ways fetching and reading a page can fail.
#[derive(Debug)]
pub enum CrawlError {
    /// The request itself failed (network error, timeout, DNS, TLS, etc.).
    Request(reqwest::Error),
    /// The server responded, but not with a 2xx status.
    BadStatus(reqwest::StatusCode),
    /// The response's content type wasn't HTML.
    NotHtml(String),
}

impl fmt::Display for CrawlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CrawlError::Request(e) => write!(f, "request failed: {e}"),
            CrawlError::BadStatus(status) => write!(f, "unexpected status: {status}"),
            CrawlError::NotHtml(content_type) => write!(f, "response was not HTML: {content_type}"),
        }
    }
}

impl From<reqwest::Error> for CrawlError {
    fn from(e: reqwest::Error) -> Self {
        CrawlError::Request(e)
    }
}
