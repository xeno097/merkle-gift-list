use std::{error::Error, fs};

use clap::Parser;
use merkle_tree::{self, MerkleeTree};

#[derive(Parser, Debug)]
pub struct GetRootArgs {
    /// File that stores the list of allowed names to receive a gift
    #[arg(long, default_value = "nice-list.json")]
    file: String,
}

pub fn get_root(args: GetRootArgs) -> Result<(), Box<dyn Error>> {
    let raw_list = fs::read_to_string(args.file)?;

    let gift_list: Vec<String> = serde_json::from_str(&raw_list)?;

    let merkle_tree_gift_list = MerkleeTree::new(gift_list);

    let root = merkle_tree_gift_list.get_root();

    println!("{:#?}", root);

    Ok(())
}
