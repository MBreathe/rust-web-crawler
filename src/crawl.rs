use scraper::{Html, Selector};
use url::Url;

use crate::error::CrawlError;

/// A fetched page: its (canonical) URL and the links found on it.
pub struct Page {
    pub url: Url,
    pub links: Vec<Url>,
}

/// Fetches `url` and extracts every link on the page, resolved to absolute URLs.
/// Fails if the request errors, the response isn't a 2xx, or the content isn't HTML.
pub fn fetch_page(client: &reqwest::blocking::Client, url: &Url) -> Result<Page, CrawlError> {
    let response = client.get(url.clone()).send()?;

    if !response.status().is_success() {
        return Err(CrawlError::BadStatus(response.status()));
    }

    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    if !content_type.contains("text/html") {
        return Err(CrawlError::NotHtml(content_type));
    }

    let body = response.text()?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("a[href]").unwrap();

    let mut links = Vec::new();
    for element in document.select(&selector) {
        let Some(href) = element.value().attr("href") else {
            continue;
        };
        match resolve_link(url, href) {
            Some(resolved) => links.push(resolved),
            None => eprintln!("skipping malformed link {href:?} found on {url}"),
        }
    }

    Ok(Page {
        url: url.clone(),
        links,
    })
}

/// Resolves `href` against `base` into an absolute URL with any fragment stripped.
/// Returns `None` if `href` doesn't parse as a URL at all.
pub fn resolve_link(base: &Url, href: &str) -> Option<Url> {
    let mut resolved = base.join(href).ok()?;
    resolved.set_fragment(None);
    Some(resolved)
}

/// True if `a` and `b` have the same host.
pub fn same_domain(a: &Url, b: &Url) -> bool {
    a.host_str() == b.host_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_path_resolution() {
        assert_eq!(
            resolve_link(&Url::parse("https://example.com/a/b").unwrap(), "c"),
            Some(Url::parse("https://example.com/a/c").unwrap())
        );
    }

    #[test]
    fn fragment_stripped() {
        assert_eq!(
            resolve_link(&Url::parse("https://example.com/a#fragment").unwrap(), "c"),
            Some(Url::parse("https://example.com/c").unwrap())
        );
    }

    #[test]
    fn same_domain_returns_correctly() {
        assert!(same_domain(
            &Url::parse("https://example.com/a/b").unwrap(),
            &Url::parse("https://example.com/a/b").unwrap()
        ));
        assert!(!same_domain(
            &Url::parse("https://example.com/a/b").unwrap(),
            &Url::parse("https://google.com/a/b").unwrap()
        ))
    }

    #[test]
    #[ignore] // hits the real network — run explicitly with `cargo test -- --ignored`
    fn fetch_example_com() {
        let client = reqwest::blocking::Client::new();
        let url = Url::parse("https://example.com").unwrap();
        let page = fetch_page(&client, &url).unwrap();
        println!("{:?}", page.links);
    }
}
