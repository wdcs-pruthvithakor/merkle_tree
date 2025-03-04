use crate::hasher::Hasher;
use std::collections::HashMap;

/// Represents a single item in a Merkle proof (sibling hash and direction)
pub struct ProofItem {
    /// The hash of the sibling node
    pub hash: Vec<u8>,
    /// Whether the sibling is on the left (true) or right (false)
    pub is_left: bool,
}

/// Represents a Merkle proof
pub struct MerkleProof<H: Hasher> {
    /// The leaf being proven
    pub leaf: Vec<u8>,
    /// The proof items (sibling hashes and their positions)
    pub proof_items: Vec<ProofItem>,
    /// The hasher for the proof
    pub hasher: H,
}

impl<H: Hasher> MerkleProof<H> {
    /// Creates a new Merkle proof
    pub fn new(leaf: Vec<u8>, proof_items: Vec<ProofItem>, hasher: H) -> Self {
        MerkleProof {
            leaf,
            proof_items,
            hasher,
        }
    }
    
    /// Calculates the root using the proof items with direction information
    pub fn calculate_root(&self) -> Vec<u8> {
        let mut current = self.leaf.clone();
        
        for item in &self.proof_items {
            current = if item.is_left {
                // Sibling is left, current is right
                self.hasher.hash_pair(&item.hash, &current)
            } else {
                // Sibling is right, current is left
                self.hasher.hash_pair(&current, &item.hash)
            };
        }
        
        current
    }
    
    /// Verifies the proof against a given root
    pub fn verify(&self, root: &[u8]) -> bool {
        self.calculate_root() == root
    }
    
    /// Converts the proof to a JSON-like format for debugging or serialization
    pub fn to_debug_format(&self) -> Vec<HashMap<String, String>> {
        self.proof_items.iter().map(|item| {
            let mut map = HashMap::new();
            let hash_hex = hex::encode(&item.hash);
            let direction = if item.is_left { "left" } else { "right" };
            
            map.insert("hash".to_string(), hash_hex);
            map.insert("direction".to_string(), direction.to_string());
            map
        }).collect()
    }
}
