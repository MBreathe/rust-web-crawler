use std::{
    collections::{HashMap, HashSet, VecDeque},
    thread,
    time::Duration,
};

use scraper::{Html, Selector};
use url::Url;

use crate::{error::CrawlError, robots::Robots};

/// A fetched page: its (canonical) URL and the links found on it.
pub struct Page {
    pub url: Url,
    pub links: Vec<Url>,
}

/// Fetches `url` and extracts every link on the page, resolved to absolute URLs.
/// Fails if the request errors, the response isn't a 2xx, or the content isn't HTML.
pub fn fetch_page(client: &reqwest::blocking::Client, url: &Url) -> Result<Page, CrawlError> {
    let response = client.get(url.clone()).send()?;
    let final_url = response.url().clone();

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
        url: final_url,
        links,
    })
}

/// Crawls same-domain pages breadth-first from `start_url`, returning a
/// `page -> links found on it` site map. Stops enqueueing new pages once
/// `max_pages` have been visited, and doesn't follow links past `max_depth`
/// hops from the start.
pub fn crawl(
    client: &reqwest::blocking::Client,
    start_url: &Url,
    robots: &Robots,
    max_depth: u32,
    max_pages: usize,
    delay: Duration,
) -> HashMap<Url, Vec<Url>> {
    let mut frontier: VecDeque<(Url, u32)> = VecDeque::new();
    let mut visited: HashSet<Url> = HashSet::new();
    let mut site_map: HashMap<Url, Vec<Url>> = HashMap::new();

    visited.insert(start_url.clone());
    frontier.push_back((start_url.clone(), 0));

    while let Some((url, depth)) = frontier.pop_front() {
        if !robots.is_allowed(url.path()) {
            eprintln!("skipping {url} (disallowed by robots.txt)");
            continue;
        }

        thread::sleep(delay);

        let page = match fetch_page(client, &url) {
            Ok(page) => page,
            Err(e) => {
                eprintln!("failed to fetch {url}: {e}");
                continue;
            }
        };

        let same_domain_links: Vec<Url> = page
            .links
            .into_iter()
            .filter(|link| same_domain(start_url, link))
            .collect();

        if depth < max_depth {
            for link in &same_domain_links {
                if visited.len() >= max_pages {
                    break;
                }
                if visited.insert(link.clone()) {
                    frontier.push_back((link.clone(), depth + 1));
                }
            }
        }

        site_map.insert(page.url, same_domain_links);
    }

    site_map
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
