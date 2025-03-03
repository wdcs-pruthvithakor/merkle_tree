pub mod utils;
pub mod tree;
pub mod proof;
pub mod hasher;


#[cfg(test)]
mod tests {
    use super::*;
    use crate::hasher::{Hasher, Sha256Hasher};
    
    #[test]
    fn test_merkle_tree() {
        let hasher = Sha256Hasher::new();
        let leaves = vec![
            utils::string_to_bytes("leaf1"),
            utils::string_to_bytes("leaf2"),
            utils::string_to_bytes("leaf3"),
            utils::string_to_bytes("leaf4"),
        ];
        
        let leaves = leaves.iter()
            .map(|leaf| hasher.hash_leaf(leaf))
            .collect();
        
        let tree = tree::MerkleTree::new(leaves, hasher);
        
        // Test root calculation
        let root = tree.root();
        assert!(!root.is_empty());
        
        // Test proof generation and verification
        let proof = tree.generate_proof(1).unwrap();
        assert!(tree.verify_proof(&proof));
        
        // Test against a different root
        let different_root = Sha256Hasher::new().hash_leaf(&utils::string_to_bytes("different"));
        assert_ne!(root, different_root);
        assert!(!proof.verify(&different_root));
    }
    
    #[test]
    fn test_tree_from_strings() {
        let strings = vec!["leaf1", "leaf2", "leaf3", "leaf4"];
        let tree = utils::create_tree_from_strings(strings);
        
        // Test proof generation and verification
        let proof = tree.generate_proof(2).unwrap();
        assert!(tree.verify_proof(&proof));
    }
    
    #[test]
    fn test_odd_number_of_leaves() {
        let strings = vec!["leaf1", "leaf2", "leaf3"];
        let tree = utils::create_tree_from_strings(strings);
        
        // Test proof generation and verification for each leaf
        for i in 0..3 {
            let proof = tree.generate_proof(i).unwrap();
            assert!(tree.verify_proof(&proof));
        }
    }
    
    #[test]
    fn test_single_leaf() {
        let strings = vec!["leaf1"];
        let tree = utils::create_tree_from_strings(strings);
        
        // Test proof generation and verification
        let proof = tree.generate_proof(0).unwrap();
        assert!(tree.verify_proof(&proof));
    }
    
    #[test]
    fn test_custom_hasher() {
        // Example of using a custom hasher
        use crate::hasher::Blake2bHasher;
        
        let hasher = Blake2bHasher::new(32); // 32-byte output size
        let strings = vec!["leaf1", "leaf2", "leaf3", "leaf4"];
        let tree = utils::create_tree_from_strings_with_hasher(strings, hasher);
        
        // Test proof generation and verification
        let proof = tree.generate_proof(2).unwrap();
        assert!(tree.verify_proof(&proof));
    }
}