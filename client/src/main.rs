use std::error::Error;

use client::cli::run_cli;

fn main() -> Result<(), Box<dyn Error>> {
    run_cli()
}
