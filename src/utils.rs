/// Helper functions for working with Merkle trees
use super::tree::MerkleTree;
use super::hasher::{Hasher, Sha256Hasher};

/// Converts a string to bytes
pub fn string_to_bytes(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}

/// Creates a Merkle tree from a list of strings using SHA-256 hasher
pub fn create_tree_from_strings(strings: Vec<&str>) -> MerkleTree<Sha256Hasher> {
    let hasher = Sha256Hasher::new();
    let leaves = strings.iter()
        .map(|s| hasher.hash_leaf(string_to_bytes(s).as_slice()))
        .collect();
    
    MerkleTree::new(leaves, hasher)
}

/// Creates a Merkle tree from a list of strings with a custom hasher
pub fn create_tree_from_strings_with_hasher<H: Hasher>(
    strings: Vec<&str>, 
    hasher: H
) -> MerkleTree<H> {
    let leaves = strings.iter()
        .map(|s| hasher.hash_leaf(string_to_bytes(s).as_slice()))
        .collect();
    
    MerkleTree::new(leaves, hasher)
}