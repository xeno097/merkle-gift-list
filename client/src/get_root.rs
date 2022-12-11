use std::{error::Error, fs};

use clap::Parser;
use merkle_tree::{self, MerkleeTree};

#[derive(Parser)]
pub struct GetRootArgs {}

pub fn get_root(file_path: &str) -> Result<(), Box<dyn Error>> {
    let raw_list = fs::read_to_string(file_path)?;

    let gift_list: Vec<String> = serde_json::from_str(&raw_list)?;

    let root = MerkleeTree::new(gift_list).get_root();

    println!("Merkle root: {root}");

    Ok(())
}
