/// Helper functions for working with Merkle trees
use std::collections::HashMap;
use super::tree::MerkleTree;
use super::hasher::{Hasher, Sha256Hasher};
use super::proof::MerkleProof;

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

/// Verifies an element in the tree exists
pub fn verify_element_in_tree<H: Hasher>(
    tree: &MerkleTree<H>,
    element: &str
) -> bool {
    let hasher = tree.get_hasher();
    let leaf_value = hasher.hash_leaf(string_to_bytes(element).as_slice());
    
    match tree.generate_proof_by_value(&leaf_value) {
        Ok(proof) => proof.verify(&tree.root()),
        Err(_) => false
    }
}

/// Example of using a proof in the format provided
pub fn verify_with_formatted_proof<H: Hasher>(
    root: &[u8],
    leaf: Vec<u8>,
    proof_data: Vec<HashMap<String, String>>,
    hasher: H
) -> bool {
    // Convert the formatted proof data to ProofItem
    let mut proof_items = Vec::new();
    
    for item in proof_data {
        let hash = hex::decode(item.get("hash").unwrap()).unwrap();
        let is_left = item.get("direction").unwrap() == "left";
        
        proof_items.push(crate::proof::ProofItem {
            hash,
            is_left,
        });
    }
    
    // Create the proof
    let proof = MerkleProof::new(leaf, proof_items, hasher);
    
    // Verify
    proof.verify(root)
}