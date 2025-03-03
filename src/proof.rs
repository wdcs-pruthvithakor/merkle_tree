use crate::hasher::Hasher;

/// Represents a Merkle proof
pub struct MerkleProof<H: Hasher> {
    /// The leaf being proven
    pub leaf: Vec<u8>,
    /// The proof nodes
    pub proof: Vec<Vec<u8>>,
    /// The index of the leaf in the tree
    pub leaf_index: usize,
    /// The hasher for the proof
    pub hasher: H,
}

impl<H: Hasher> MerkleProof<H> {
    /// Creates a new Merkle proof
    pub fn new(leaf: Vec<u8>, proof: Vec<Vec<u8>>, leaf_index: usize, hasher: H) -> Self {
        MerkleProof {
            leaf,
            proof,
            leaf_index,
            hasher,
        }
    }
    
    /// Calculates the root given the proof
    pub fn calculate_root(&self) -> Vec<u8> {
        let mut current = self.leaf.clone();
        let mut current_index = self.leaf_index;
        
        for sibling in &self.proof {
            current = if current_index % 2 == 0 {
                // If current node is left child, hash(current + sibling)
                self.hasher.hash_pair(&current, sibling)
            } else {
                // If current node is right child, hash(sibling + current)
                self.hasher.hash_pair(sibling, &current)
            };
            
            current_index /= 2;
        }
        
        current
    }
    
    /// Verifies the proof against a given root
    pub fn verify(&self, root: &[u8]) -> bool {
        self.calculate_root() == root
    }
}