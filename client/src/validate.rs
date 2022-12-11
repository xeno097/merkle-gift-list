use std::{error::Error, fs};

use clap::Parser;
use merkle_tree::{MerkleTreeProofNode, MerkleeTree};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
pub struct ValidateArgs {
    /// Host of the server that will validate the data
    #[arg(long, default_value = "http://localhost:3000")]
    host: String,

    /// Name of the person to validate if it is in the list
    #[arg(long)]
    name: String,
}

#[derive(Serialize)]
struct ValidateBody {
    name: String,
    proof: Vec<MerkleTreeProofNode>,
}

#[derive(Deserialize)]
struct ValidationResponse {
    result: String,
}

pub fn validate(file_path: &str, args: ValidateArgs) -> Result<(), Box<dyn Error>> {
    let raw_list = fs::read_to_string(file_path)?;

    let gift_list: Vec<String> = serde_json::from_str(&raw_list)?;

    let idx = gift_list
        .iter()
        .enumerate()
        .find(|(_, value)| value == &&args.name)
        .map_or(gift_list.len() + 1, |(idx, _)| idx);

    let request_body = ValidateBody {
        name: args.name,
        proof: MerkleeTree::new(gift_list).get_proof(idx),
    };

    let res: ValidationResponse = reqwest::blocking::Client::new()
        .post(format!("{}/gift", args.host))
        .body(serde_json::to_string(&request_body)?)
        .send()?
        .json()?;

    println!("Validation result: {}", res.result);

    Ok(())
}
