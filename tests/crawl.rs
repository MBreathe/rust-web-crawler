use rust_web_crawler::{crawl, robots::Robots};
use std::{collections::HashMap, thread, time::Duration};
use url::Url;

/// Spawns a background HTTP server that serves the given `path -> HTML body` pages
/// (anything else 404s). Returns the server's base URL.
fn spawn_test_server(pages: HashMap<&'static str, &'static str>) -> String {
    let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
    let addr = server.server_addr();
    let base_url = format!("http://{addr}");

    thread::spawn(move || {
        for request in server.incoming_requests() {
            let (body, status) = match pages.get(request.url()) {
                Some(body) => (*body, 200),
                None => ("not found", 404),
            };
            let header =
                tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap();
            let response = tiny_http::Response::from_string(body)
                .with_status_code(status)
                .with_header(header);
            let _ = request.respond(response);
        }
    });

    base_url
}

#[test]
fn crawl_dedupes_and_stays_in_domain() {
    let pages = HashMap::from([
        (
            "/",
            r#"<a href="/b">b</a> <a href="https://example.com/off-domain">off</a>"#,
        ),
        ("/b", r#"<a href="/">a</a> <a href="/c">c</a>"#),
        ("/c", r#"<a href="/">a</a>"#),
    ]);

    let base_url = spawn_test_server(pages);
    let start_url = Url::parse(&base_url).unwrap();

    let client = reqwest::blocking::Client::new();
    let robots = Robots::default();
    let site_map = crawl::crawl(&client, &start_url, &robots, 2, 10, Duration::ZERO);

    let actual: HashMap<Url, Vec<Url>> = site_map.into_iter().collect();

    assert_eq!(actual.len(), 3);
    assert_eq!(actual[&start_url], vec![start_url.join("/b").unwrap()]);
    assert_eq!(
        actual[&start_url.join("/b").unwrap()],
        vec![start_url.clone(), start_url.join("/c").unwrap()]
    );
    assert_eq!(
        actual[&start_url.join("/c").unwrap()],
        vec![start_url.clone()]
    );
}
