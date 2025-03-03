// hasher.rs

/// Trait for hash functions used in the Merkle tree
pub trait Hasher: Clone {
    /// Hashes a leaf before inserting it into the tree
    fn hash_leaf(&self, data: &[u8]) -> Vec<u8>;
    
    /// Hashes two nodes together to create a parent node
    fn hash_pair(&self, left: &[u8], right: &[u8]) -> Vec<u8>;
}

// Default implementation using SHA-256
#[derive(Clone)]
pub struct Sha256Hasher;

impl Sha256Hasher {
    pub fn new() -> Self {
        Sha256Hasher
    }
}

impl Hasher for Sha256Hasher {
    fn hash_leaf(&self, data: &[u8]) -> Vec<u8> {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
    
    fn hash_pair(&self, left: &[u8], right: &[u8]) -> Vec<u8> {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(left);
        hasher.update(right);
        hasher.finalize().to_vec()
    }
}

// Example of a configurable hasher implementation
#[derive(Clone)]
pub struct Blake2bHasher {
    // Configuration parameters
    output_size: usize,
}

impl Blake2bHasher {
    pub fn new(output_size: usize) -> Self {
        Blake2bHasher { output_size }
    }
}

impl Hasher for Blake2bHasher {
    fn hash_leaf(&self, data: &[u8]) -> Vec<u8> {
        use blake2::{Blake2b, Digest};
        let mut hasher = Blake2b::<blake2::digest::consts::U64>::new();
        hasher.update(data);
        hasher.finalize().to_vec()[..self.output_size].to_vec()
    }
    
    fn hash_pair(&self, left: &[u8], right: &[u8]) -> Vec<u8> {
        use blake2::{Blake2b, Digest};
        let mut hasher = Blake2b::<blake2::digest::consts::U64>::new();
        hasher.update(left);
        hasher.update(right);
        hasher.finalize().to_vec()[..self.output_size].to_vec()
    }
}