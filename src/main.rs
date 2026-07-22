mod args;
mod crawl;
mod error;
mod robots;

use clap::Parser;
use std::time::Duration;
use url::Url;

fn main() {
    let args = args::Args::parse();

    let start_url = Url::parse(&args.start_url).expect("invalid start URL");
    let client = reqwest::blocking::Client::builder()
        .user_agent("rust-web-crawler/0.1 (educational Rust learning project)")
        .build()
        .expect("failed to build HTTP client");

    let robots = robots::fetch(&client, &start_url);

    let site_map = crawl::crawl(
        &client,
        &start_url,
        &robots,
        args.max_depth,
        args.max_pages,
        Duration::from_millis(args.delay_ms),
    );

    println!("{site_map:#?}");
}
