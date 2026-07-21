use clap::Parser;

/// A polite, single-domain web crawler.
#[derive(Parser, Debug)]
pub struct Args {
    /// The URL to start crawling from
    pub start_url: String,

    /// Maximum link-depth to follow from the start URL
    #[arg(long, default_value_t = 3)]
    pub max_depth: u32,

    /// Maximum number of pages to crawl before stopping
    #[arg(long, default_value_t = 100)]
    pub max_pages: usize,

    /// Delay in milliseconds between requests
    #[arg(long, default_value_t = 200)]
    pub delay_ms: u64,
}
