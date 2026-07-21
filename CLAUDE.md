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

M1 (single-threaded crawler) is in progress; nothing is implemented yet. M2 (worker pool
parallelization) and M3 (stretch goals) haven't been started — don't jump ahead to them until M1 works
and is tested.

## Architecture

Not yet written — update this section as M1/M2 land, following the same style as
`rust-expense-tracker/CLAUDE.md` (one bullet per module: what it does, and any non-obvious *why*
behind a design choice, not a restatement of what the code visibly does).

## Commands

- `cargo build` / `cargo run -- <args>` — compile / run the crawler
- `cargo test` — run tests
- `cargo clippy --all-targets` — lint
