# Merkle Tree Implementation

This project provides a Rust implementation of a **Merkle Tree**, which is a binary tree used to efficiently verify data integrity. It includes utilities for generating Merkle proofs and verifying them, as well as helpers for hashing and creating trees from strings.

## Features

- **Merkle Tree Structure**: A binary tree where each non-leaf node is the hash of its children, and leaf nodes represent hashed data.
- **Merkle Proof Generation**: Allows generating proofs for individual leaves to verify their inclusion in the tree.
- **Proof Verification**: Verifies the validity of a Merkle proof against the root.
- **Support for Odd Number of Leaves**: Handles cases where the number of leaves is odd by duplicating the last leaf to make the number of leaves a power of two.
- **Custom Hasher Support**: Users can implement their own hashing algorithm by defining a custom hasher that implements the `Hasher` trait.

## Directory Structure

- `src/`
  - `lib.rs`: Entry point for the library module, exposing Merkle tree functionality and tests.
  - `tree.rs`: Contains the implementation of the `MerkleTree` struct, which builds the tree and supports proof generation and verification.
  - `proof.rs`: Contains the `MerkleProof` struct that handles the generation and verification of Merkle proofs.
  - `utils.rs`: Provides helper functions for hashing leaves and creating trees from strings.
  - `hasher.rs`: Defines the `Hasher` trait, allowing users to implement custom hashing functions.
  - `main.rs`: A demonstration of how to use the library to create a tree and verify proofs.
  
## Usage

1. **Add dependencies**  
   Add the following to your `Cargo.toml` file to use the necessary libraries:

   ```toml
   [dependencies]
   sha2 = "0.10"
   blake2 = "0.10"
   hex = "0.4"
   ```

2. **Create a Merkle Tree**  
   You can create a Merkle tree from a list of strings using the utility functions provided:

   ```rust
   use merkle_tree::utils;

   fn main() {
       let data = vec!["Create", "a", "tree", "from", "strings"];
       let tree = utils::create_tree_from_strings(data);
   
       let root = tree.root();
       println!("Merkle Root: {:?}", hex::encode(&root));
   }
   ```

3. **Generate and Verify a Proof**  
   To generate a proof for a specific leaf and verify it:

   ```rust
   use merkle_tree::utils;

   fn main() {
       let data = vec!["Create", "a", "tree", "from", "strings"];
       let tree = utils::create_tree_from_strings(data);
   
       let proof = tree.generate_proof(1).unwrap();
       let is_valid = tree.verify_proof(&proof);
   
       println!("Proof is valid: {}", is_valid);
   }
   ```

4. **Calculate and Verify Root from Proof**  
   To calculate the root from a proof and verify it against the root of the tree:

   ```rust
   use merkle_tree::utils;

   fn main() {
       let data = vec!["Create", "a", "tree", "from", "strings"];
       let tree = utils::create_tree_from_strings(data);
   
       let proof = tree.generate_proof(1).unwrap();
       let calculated_root = proof.calculate_root();
       let root = tree.root();
   
       println!("Calculated Root: {:?}", hex::encode(&calculated_root));
       println!("Proof verifies against root: {}", proof.verify(&root));
   }
   ```

5. **Use a Custom Hasher**  
   Users can implement their own hashing algorithms by defining a struct that implements the `Hasher` trait:

   ```rust
   use merkle_tree::hasher::{Hasher, Sha256Hasher};
   use merkle_tree::utils;

   #[derive(Clone)]
   struct MyCustomHasher;

   impl Hasher for MyCustomHasher {
       fn hash_leaf(&self, data: &[u8]) -> Vec<u8> {
           // Custom hash function for leaf
           data.to_vec()
       }
       
       fn hash_pair(&self, left: &[u8], right: &[u8]) -> Vec<u8> {
           // Custom hash function for parent nodes
           [left, right].concat()
       }
   }

   fn main() {
       let data = vec!["Create", "a", "tree", "from", "strings"];
       let custom_hasher = MyCustomHasher;
       let tree = utils::create_tree_from_strings_with_hasher(data, custom_hasher);
   
       let root = tree.root();
       println!("Merkle Root with custom hasher: {:?}", hex::encode(&root));
   }
   ```

## Tests

The project includes tests to verify the correctness of the Merkle tree functionality, including:

- **Merkle Tree creation** and root calculation.
- **Proof generation and verification** for different cases.
- Handling of **odd-numbered leaves** in the tree.
- Case for **single leaf trees**.
- **Custom hasher support** with user-defined implementations.

### Running Tests

To run the tests, use the following command:

```bash
cargo test
```

## Example Output

```bash
Merkle Root: <root_hash_in_hex>
Proof is valid: true
Calculated Root: <calculated_root_in_hex>
Proof verifies against root: true
Merkle Root with custom hasher: <custom_root_hash>
```

## Files

- **`tree.rs`**: Contains the `MerkleTree` struct, its construction, and methods to interact with the tree.
- **`proof.rs`**: Defines the `MerkleProof` struct for generating and verifying Merkle proofs.
- **`utils.rs`**: Helper functions for hashing and creating trees from strings.
- **`hasher.rs`**: Defines the `Hasher` trait and includes default implementations (SHA-256 and Blake2b).
- **`main.rs`**: Demonstrates how to use the Merkle tree and generate/verify proofs.
  
## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

