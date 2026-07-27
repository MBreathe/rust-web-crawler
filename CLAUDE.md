# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

A single-domain web crawler, built as a staged Rust learning project focused on concurrency/OS-thread
programming (see README.md for the roadmap, and
`docs/superpowers/specs/2026-07-21-web-crawler-design.md` for the full design). This is a learning
exercise, not production code — prefer explaining *why* Rust idioms (and concurrency patterns
specifically) work the way they do over silently writing code for the user. When the user is
implementing a new concept themselves, explain and show snippets for them to type in rather than
writing/editing the implementation files directly; direct edits are fine for mechanical setup
(Cargo.toml deps, `.gitignore`, config, docs).

M1 (single-threaded crawler) is done and tested. M2 (worker pool parallelization) and M3 (stretch
goals) haven't been started — don't jump ahead to them until asked.

## Architecture

The crate is split bin+lib (`src/lib.rs` exposes `args`/`crawl`/`error`/`robots`; `src/main.rs` is a
thin binary), the same reason as `rust-expense-tracker`: integration tests under `tests/` compile as
separate crates and can only reach a library's public surface, not a binary's private `mod`s.

- **`args.rs`** — `clap`-derived `Args`: positional `start_url`, plus `--max-depth`/`--max-pages`/
  `--delay-ms` flags with defaults. Doc comments on the fields double as `--help` text.
- **`error.rs`** — `CrawlError` (`Request`/`BadStatus`/`NotHtml`), covering the ways fetching and
  reading one page can fail. `From<reqwest::Error>` lets `?` convert a network failure automatically.
- **`robots.rs`** — `Robots` holds only `Disallow:` paths, collected across the whole file regardless
  of which `User-agent` block they're under (a deliberate simplification, not full robots.txt spec
  compliance). `Robots::default()` (empty rules) doubles as the "missing/unreachable robots.txt"
  fallback, since both cases mean the same thing: no restrictions. `fetch` takes the shared
  `reqwest::blocking::Client` (not a one-off `get`), so the `User-Agent` politeness setting applies to
  robots.txt too. Uses `start_url.join("/robots.txt")` — the leading slash matters, since a bare
  `"robots.txt"` resolves relative to the start URL's *path*, not the domain root.
- **`crawl.rs`** — the whole crawl:
  - `resolve_link`/`same_domain` — small `Url`-handling helpers, unit tested in isolation.
  - `Page`/`fetch_page` — fetches one URL, extracts every `<a href>`, resolves each to an absolute
    URL. `Page.url` is the response's *final* URL (`response.url()`, after any redirects), not the
    requested one — that's what lets two different requested URLs that redirect to the same page
    collapse to one site-map entry instead of two.
  - `crawl` — the BFS loop: `VecDeque` frontier (FIFO, not a stack, hence breadth-first),
    `HashSet<Url>` visited set (a URL is marked visited the moment it's *enqueued*, not fetched — that's
    what prevents two different pages linking to the same URL from queuing it twice), returns
    `Vec<(Url, Vec<Url>)>` rather than a `HashMap` specifically so the crawl order is preserved for
    printing (`HashMap` iteration order is arbitrary). `same_domain` is always checked against the
    original `start_url`, never the current page, so the domain boundary can't drift.  `max_pages`
    stops *enqueueing* new work once hit (checked inside the enqueue loop), not the whole crawl
    mid-frontier, so it winds down cleanly rather than cutting off abruptly.

## Testing

- Unit tests (`resolve_link`/`same_domain` in `crawl.rs`, `Robots::parse`/`is_allowed` in `robots.rs`)
  live inline in `#[cfg(test)] mod tests` blocks.
- `tests/crawl.rs` — a real integration test: spins up a background `tiny_http` server
  (`127.0.0.1:0`, so the OS picks a free port) serving a small fixed link graph with a deliberate
  cycle and an off-domain link, then asserts the crawl's output matches exactly — proving dedup
  (the cycle doesn't cause a re-crawl or infinite loop) and domain-scope filtering (the off-domain
  link never appears) in one test. `tiny_http` is a `[dev-dependencies]`-only, synchronous server
  library — no async runtime needed, consistent with the rest of the (blocking-only) codebase.
- One `#[ignore]` test in `crawl.rs` (`fetch_example_com`) hits the real network against the stable
  `example.com` test domain — run explicitly with `cargo test -- --ignored`, not part of the default
  suite.

## Commands

- `cargo build` / `cargo run -- <args>` — compile / run the crawler
- `cargo test` — run tests
- `cargo clippy --all-targets` — lint
