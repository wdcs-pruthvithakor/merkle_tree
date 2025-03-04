// tree.rs

use std::collections::HashMap;
use crate::proof::{MerkleProof, ProofItem};
use crate::hasher::Hasher;

/// Represents a Merkle tree data structure
pub struct MerkleTree<H: Hasher> {
    /// The leaves of the tree
    leaves: Vec<Vec<u8>>,
    /// The cached nodes of the tree, indexed by level and position
    nodes: HashMap<(usize, usize), Vec<u8>>,
    /// The height of the tree
    height: usize,
    /// The hasher for the tree
    hasher: H,
}

impl<H: Hasher> MerkleTree<H> {
    /// Creates a new Merkle tree with a specific hasher
    pub fn new(mut leaves: Vec<Vec<u8>>, hasher: H) -> Self {
        if leaves.is_empty() {
            panic!("Cannot create a Merkle tree with no leaves");
        }

        leaves.sort();

        let mut tree = MerkleTree {
            leaves: leaves.clone(),
            nodes: HashMap::new(),
            height: 0,
            hasher,
        };
        
        // Calculate the height of the tree
        // The height is log2(next_power_of_2(leaves.len())) + 1
        let next_power_of_2 = if leaves.len().is_power_of_two() {
            leaves.len()
        } else {
            leaves.len().next_power_of_two()
        };
        
        tree.height = next_power_of_2.trailing_zeros() as usize + 1;
        
        // Build the tree
        tree.build();
        
        tree
    }
    
    /// Builds the Merkle tree
    fn build(&mut self) {
        // Extend leaves to the next power of 2 if necessary
        let target_length = 1 << (self.height - 1);
        
        if self.leaves.len() < target_length {
            let last_leaf = self.leaves.last().unwrap().clone();
            while self.leaves.len() < target_length {
                self.leaves.push(last_leaf.clone());
            }
        }
        
        // Add leaves to the nodes map
        for (i, leaf) in self.leaves.iter().enumerate() {
            self.nodes.insert((0, i), leaf.clone());
        }
        
        // Build the tree from bottom to top
        for level in 0..self.height - 1 {
            let next_level_width = 1 << (self.height - 2 - level);
            for i in 0..next_level_width {
                let left = self.nodes.get(&(level, i * 2)).unwrap().clone();
                let right = self.nodes.get(&(level, i * 2 + 1)).unwrap().clone();
                
                let parent = self.hash_pair(&left, &right);
                self.nodes.insert((level + 1, i), parent);
            }
        }
    }
    
    /// Gets the root of the Merkle tree
    pub fn root(&self) -> Vec<u8> {
        self.nodes.get(&(self.height - 1, 0)).unwrap().clone()
    }
    
    /// Gets the leaf at the given index
    pub fn get_leaf(&self, index: usize) -> Option<&Vec<u8>> {
        self.leaves.get(index)
    }

    /// get the hasher of the tree
    pub fn get_hasher(&self) -> H {
        self.hasher.clone()
    }

    /// Gets the number of leaves in the tree
    pub fn leaf_count(&self) -> usize {
        self.leaves.len()
    }
    
    /// Finds the leaf index for a given leaf value
    pub fn find_leaf_index(&self, leaf_value: &[u8]) -> Option<usize> {
        self.leaves.iter().position(|leaf| leaf == leaf_value)
    }
    
    /// Generates a Merkle proof for the leaf at the given index
    pub fn generate_proof(&self, leaf_index: usize) -> Result<MerkleProof<H>, &'static str> {
        if leaf_index >= self.leaves.len() {
            return Err("Leaf index out of bounds");
        }
        
        let mut proof_items = Vec::new();
        let mut current_index = leaf_index;
        
        for level in 0..self.height - 1 {
            let is_right_child = current_index % 2 == 1;
            let sibling_index = if is_right_child {
                current_index - 1  // Sibling is on the left
            } else {
                current_index + 1  // Sibling is on the right
            };
            
            if let Some(sibling) = self.nodes.get(&(level, sibling_index)) {
                proof_items.push(ProofItem {
                    hash: sibling.clone(),
                    is_left: is_right_child,  // If current is right, sibling is left
                });
            } else {
                // If the sibling doesn't exist (at the edge of an odd-length level),
                // use the current node as its own sibling but with appropriate direction
                let current_node = self.nodes.get(&(level, current_index)).unwrap().clone();
                proof_items.push(ProofItem {
                    hash: current_node,
                    is_left: is_right_child,
                });
            }
            
            current_index /= 2;
        }
        
        Ok(MerkleProof::new(
            self.leaves[leaf_index].clone(),
            proof_items,
            self.hasher.clone(),
        ))
    }
    
    /// Generates a Merkle proof for the given leaf value
    pub fn generate_proof_by_value(&self, leaf_value: &[u8]) -> Result<MerkleProof<H>, &'static str> {
        if let Some(index) = self.find_leaf_index(leaf_value) {
            self.generate_proof(index)
        } else {
            Err("Leaf value not found in the tree")
        }
    }
    
    /// Verifies a Merkle proof
    pub fn verify_proof(&self, proof: &MerkleProof<H>) -> bool {
        let calculated_root = proof.calculate_root();
        self.root() == calculated_root
    }

    
    /// Hashes two nodes together to create a parent node
    fn hash_pair(&self, left: &[u8], right: &[u8]) -> Vec<u8> {
        self.hasher.hash_pair(left, right)
    }
}
