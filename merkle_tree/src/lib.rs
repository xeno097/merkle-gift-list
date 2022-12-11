use serde::{Deserialize, Serialize};
// Code copied asn adapted from
// https://github.com/xeno097/ethereum-bootcamp-rust-edition/blob/main/src/week_2/merkle_tree.rs
use sha3::Digest;

pub struct MerkleeTree {
    leaves: Vec<String>,
}

fn keccak256(data: impl AsRef<[u8]>) -> String {
    let mut hasher = sha3::Keccak256::new();
    hasher.update(data);
    let hash: Vec<u8> = hasher.finalize().into_iter().collect();

    hex::encode(hash)
}

pub fn merge(left: impl AsRef<[u8]>, right: impl AsRef<[u8]>) -> String {
    let mut hasher = sha3::Keccak256::new();

    hasher.update(left);
    hasher.update(right);

    let hash: Vec<u8> = hasher.finalize().into_iter().collect();

    hex::encode(hash)
}

#[derive(Serialize, Deserialize)]
pub struct MerkleTreeProofNode {
    data: String,
    is_left: bool,
}

impl MerkleeTree {
    pub fn new(leaves: Vec<String>) -> Self {
        if leaves.is_empty() {
            panic!("Cannot build merkle tree from empty list")
        }

        Self { leaves }
    }

    pub fn get_root(&self) -> String {
        let hashed_leaves: Vec<String> = self.leaves.iter().map(keccak256).collect();

        MerkleeTree::build_root(&hashed_leaves)
    }

    fn build_root(level: &[String]) -> String {
        let level_size = level.len();

        if level_size == 1 {
            return level.get(0).unwrap().clone();
        }

        let new_level: Vec<String> = level
            .chunks(2)
            .map(|chunk| {
                if chunk.len() == 1 {
                    return chunk.get(0).unwrap().clone();
                }

                let left = chunk.get(0).unwrap();
                let right = chunk.get(1).unwrap();

                merge(left, right)
            })
            .collect();

        MerkleeTree::build_root(&new_level)
    }

    pub fn get_proof(&self, idx: usize) -> Vec<MerkleTreeProofNode> {
        let hashed_leaves: Vec<String> = self.leaves.iter().map(keccak256).collect();

        MerkleeTree::build_proof(idx, &hashed_leaves)
    }

    fn build_proof(idx: usize, level: &[String]) -> Vec<MerkleTreeProofNode> {
        let level_size = level.len();
        let mut partial_proof = Vec::new();

        if level_size == 1 {
            return partial_proof;
        }

        let mut counter = 0;
        let new_level: Vec<String> = level
            .chunks(2)
            .map(|chunk| {
                if chunk.len() == 1 {
                    return chunk.get(0).unwrap().clone();
                }

                let left = chunk.get(0).unwrap();
                let right = chunk.get(1).unwrap();

                if counter == idx || counter == idx.saturating_sub(1) {
                    let is_target_at_left = idx % 2 == 0;

                    let curr_proof_node = MerkleTreeProofNode {
                        data: if is_target_at_left {
                            right.clone()
                        } else {
                            left.clone()
                        },
                        is_left: !is_target_at_left,
                    };

                    partial_proof.push(curr_proof_node);
                }

                counter += 2;
                merge(left, right)
            })
            .collect();

        partial_proof.append(MerkleeTree::build_proof(idx / 2, &new_level).as_mut());

        partial_proof
    }
}

pub fn validate(value: String, root: String, proof: Vec<MerkleTreeProofNode>) -> bool {
    let candidate_root = proof.iter().fold(keccak256(value), |acc, proof_node| {
        if proof_node.is_left {
            merge(&proof_node.data, acc)
        } else {
            merge(acc, &proof_node.data)
        }
    });

    root == candidate_root
}
