# rust-web-crawler

A single-domain web crawler, built as a learning project for Rust.

The goal isn't a finished product — it's a staged path through Rust's
concurrency model (OS threads, channels, shared state) using a project simple
enough to reason about but real enough to run against actual sites. See
`docs/superpowers/specs/2026-07-21-web-crawler-design.md` for the full design.

## Roadmap

**M1 — Single-threaded crawler** (done)
Breadth-first crawl of a single domain: fetch a page, extract its links,
filter to same-domain, recurse into unvisited ones. Covers: `reqwest`
(blocking HTTP), `scraper` (HTML parsing), `url` (parsing/normalizing links),
and politeness basics (robots.txt, rate limiting) from day one, since it
already talks to real servers.

**M2 — Worker pool** (not started)
The same crawl logic, parallelized across a fixed pool of OS threads: a shared
job queue (`Arc<Mutex<Receiver<Url>>>`), shared visited-set/result-map state
(`Arc<Mutex<...>>`), and termination detection for a queue whose work is
discovered dynamically (an `Arc<AtomicUsize>` in-flight counter). Benchmarked
against M1 on the same site.

**M3 — Stretch goals** (pick as interest allows)
Graphviz `.dot` output, swapping the hand-rolled queue for `crossbeam-channel`,
respecting `Crawl-delay` from robots.txt, per-domain rate limiting if scope
ever widens beyond one domain.

## Data model

- **Frontier**: URLs discovered but not yet crawled.
- **Visited set**: URLs already fetched, to avoid re-crawling.
- **Site map**: `page -> [links found on it]`, the crawl's final output.

## Scope and politeness

The crawler only follows links within the domain it was started on — it
never wanders off to external sites. Every crawl fetches and respects
`/robots.txt`, waits a configurable delay between requests, and identifies
itself with an honest `User-Agent`. `--max-depth` and `--max-pages` bound
every crawl so it always terminates.

## Usage

```
cargo run -- <start-url> [--max-depth N] [--max-pages N] [--delay-ms N]
```

Prints one line per crawled page: `page -> [link1, link2, ...]`, in crawl order.

## Status

M1 is done: a single-threaded, same-domain, robots.txt-respecting crawler with an integration test
covering cycle dedup and domain-scope filtering. M2 (worker pool) hasn't been started.
