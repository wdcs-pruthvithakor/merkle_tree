/// Helper functions for working with Merkle trees
use super::tree::MerkleTree;
use sha2::{Digest, Sha256};
    
/// Converts a string to bytes
pub fn string_to_bytes(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}

/// Hashes a leaf before inserting it into the tree
pub fn hash_leaf(leaf: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(leaf);
    hasher.finalize().to_vec()
}

/// Creates a Merkle tree from a list of strings
pub fn create_tree_from_strings(strings: Vec<&str>) -> MerkleTree {
    let leaves = strings.iter()
        .map(|s| hash_leaf(string_to_bytes(s).as_slice()))
        .collect();
    
    MerkleTree::new(leaves)
}