use merkle_tree::utils;

fn main() {
    // Create a tree from strings using the default SHA-256 hasher
    let data = vec!["Create", "a", "tree", "from", "strings"];
    let tree = utils::create_tree_from_strings(data);
    
    // Get the root of the tree
    let root = tree.root();
    println!("Merkle Root: {:?}", hex::encode(&root));
    
    // Generate a proof for transaction2
    let proof = tree.generate_proof(1).unwrap();
    
    // Verify the proof
    let is_valid = tree.verify_proof(&proof);
    println!("Proof is valid: {}", is_valid);
    
    // Calculate the root from the proof
    let calculated_root = proof.calculate_root();
    println!("Calculated Root: {:?}", hex::encode(&calculated_root));
    
    // Verify the proof against the root
    let is_valid = proof.verify(&root);
    println!("Proof verifies against root: {}", is_valid);
    
    // Example of using a custom hasher
    // let custom_hasher = merkle_tree::hasher::Blake2bHasher::new(32);
    // let custom_tree = utils::create_tree_from_strings_with_hasher(data, custom_hasher);
}