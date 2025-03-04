use merkle_tree::{hasher::Hasher, utils};

fn main() {
    // Create a tree from strings using the default SHA-256 hasher
    let data = vec!["Create", "a", "tree", "from", "strings"];
    let tree = utils::create_tree_from_strings(data);
    
    // Get the root of the tree
    let root = tree.root();
    println!("Merkle Root: {:?}", hex::encode(&root));
    
    // Generate a proof for 'from'
    let proof = tree.generate_proof(3).unwrap();
    
    // Verify the proof
    let is_valid = tree.verify_proof(&proof);
    println!("Proof is valid: {}", is_valid);
    
    // Calculate the root from the proof
    let calculated_root = proof.calculate_root();
    println!("Calculated Root: {:?}", hex::encode(&calculated_root));
    
    // Verify the proof against the root
    let is_valid = proof.verify(&root);
    println!("Proof verifies against root: {}", is_valid);

    // Verify the proof if 'from' is present in tree
    let is_present = utils::verify_element_in_tree(&tree, "from");
    println!("Proof verifies against presence of element in tree: {}", is_present);

    // Verify the proof if leaf is present in tree with given proof data
    let leaf = tree.get_hasher().hash_leaf(utils::string_to_bytes("from").as_slice());
    let proof_data = tree.generate_proof_by_value(&leaf).expect("Couldn't generate proof").to_debug_format();
    println!("{:#?}", proof_data);
    let is_valid = utils::verify_with_formatted_proof(&tree.root(), leaf, proof_data, tree.get_hasher());
    println!("Proof verify agaist given proof data: {}", is_valid);

    // Example of using a custom hasher
    // let custom_hasher = merkle_tree::hasher::Blake2bHasher::new(32);
    // let custom_tree = utils::create_tree_from_strings_with_hasher(data, custom_hasher);
}