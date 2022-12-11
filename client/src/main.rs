use std::{error::Error, fs};

use clap::{Command, FromArgMatches, Parser, Subcommand};
use merkle_tree::{self, MerkleeTree};

#[derive(Parser, Debug)]
struct GetRootArgs {
    /// File that stores the list of allowed names to receive a gift
    #[arg(long, default_value = "nice-list.json")]
    file: String,
}

#[derive(Parser, Debug)]
struct ValidateArgs {
    /// File that stores the list of allowed names to receive a gift
    #[arg(long, default_value = "nice-list.json")]
    file: String,

    /// Host of the server that will validate the data
    #[arg(long, default_value = "127.0.0.1:3000")]
    host: String,

    /// Name of the person to validate if it is in the list
    #[arg(long)]
    name: String,
}

#[derive(Parser, Debug)]
enum Subcommands {
    GetRoot(GetRootArgs),
    Validate(ValidateArgs),
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Command::new("Built CLI");

    let cli = Subcommands::augment_subcommands(cli);

    let matches = cli.get_matches();
    let derived_subcommands = Subcommands::from_arg_matches(&matches)?;

    match derived_subcommands {
        Subcommands::GetRoot(args) => {
            let raw_list = fs::read_to_string(args.file)?;

            let gift_list: Vec<String> = serde_json::from_str(&raw_list)?;

            let merkle_tree_gift_list = MerkleeTree::new(gift_list);

            let root = merkle_tree_gift_list.get_root();

            println!("{:#?}", root);
        }
        Subcommands::Validate(_) => todo!(),
    }

    Ok(())
}
