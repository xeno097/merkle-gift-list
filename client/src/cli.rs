use std::error::Error;

use clap::Parser;

use crate::{
    get_root::{get_root, GetRootArgs},
    validate::{validate, ValidateArgs},
};

#[derive(Parser)]
enum Commands {
    /// Builds a merkle root from <FILE>
    GetRoot(GetRootArgs),

    /// Validates that <NAME> belongs to the merkle tree by querying <HOST>
    Validate(ValidateArgs),
}

#[derive(Parser)]
struct Cli {
    /// File that stores the list of allowed names to receive a gift
    #[arg(short, long, default_value = "nice-list.json")]
    file: String,

    #[clap(subcommand)]
    subcommand: Commands,
}

pub fn run_cli() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match args.subcommand {
        Commands::GetRoot(_) => get_root(&args.file),
        Commands::Validate(validate_args) => validate(&args.file, validate_args),
    }
}
