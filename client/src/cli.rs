use std::error::Error;

use clap::{Command, FromArgMatches, Parser, Subcommand};

use crate::{
    get_root::{get_root, GetRootArgs},
    validate::ValidateArgs,
};

#[derive(Parser, Debug)]
enum Subcommands {
    GetRoot(GetRootArgs),
    Validate(ValidateArgs),
}

pub fn run_cli() -> Result<(), Box<dyn Error>> {
    let cli = Command::new("Built CLI");

    let cli = Subcommands::augment_subcommands(cli);

    let matches = cli.get_matches();
    let derived_subcommands = Subcommands::from_arg_matches(&matches)?;

    match derived_subcommands {
        Subcommands::GetRoot(args) => get_root(args),
        Subcommands::Validate(_) => todo!(),
    }
}
