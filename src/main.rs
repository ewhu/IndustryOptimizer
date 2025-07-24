// src/main.rs
/*
 * Main executable for IndustryOptimizer
 */

use clap::Parser;
use industryoptimizer::{Result, run};

#[derive(Parser)]
#[command(version, about = "IndustryOptimizer - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
