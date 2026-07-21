mod args;
mod crawl;
mod error;
mod robots;

use clap::Parser;

fn main() {
    let args = args::Args::parse();
    println!("{args:?}");
}
