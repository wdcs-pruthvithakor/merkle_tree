/// Represents a Merkle proof
use sha2::{Digest, Sha256};

pub struct MerkleProof {
    /// The leaf being proven
    pub leaf: Vec<u8>,
    /// The proof nodes
    pub proof: Vec<Vec<u8>>,
    /// The index of the leaf in the tree
    pub leaf_index: usize,
}

impl MerkleProof {
    /// Calculates the root given the proof
    pub fn calculate_root(&self) -> Vec<u8> {
        let mut current = self.leaf.clone();
        let mut current_index = self.leaf_index;
        
        for sibling in &self.proof {
            let mut hasher = Sha256::new();
            
            if current_index % 2 == 0 {
                hasher.update(&current);
                hasher.update(sibling);
            } else {
                hasher.update(sibling);
                hasher.update(&current);
            }
            
            current = hasher.finalize().to_vec();
            current_index /= 2;
        }
        
        current
    }
    
    /// Verifies the proof against a given root
    pub fn verify(&self, root: &[u8]) -> bool {
        self.calculate_root() == root
    }
}
