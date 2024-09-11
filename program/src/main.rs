#![no_main]
sp1_zkvm::entrypoint!(main);

use celestia_types::{nmt::Namespace, Blob};
use sha3::{Digest, Keccak256};

pub fn main() {
    // read the namespace
    let namespace: Namespace = sp1_zkvm::io::read();

    // Read the blob data
    let data = sp1_zkvm::io::read_vec();

    // Create a blob
    let blob = Blob::new(namespace, data).expect("Failed to create blob");

    // Compute the commitment
    let commitment = blob.commitment;

    // Compute the keccak256 hash of the blob
    let mut hasher = Keccak256::new();
    hasher.update(&blob.data);
    let hash = hasher.finalize();

    // Output the commitment bytes
    sp1_zkvm::io::commit_slice(&commitment.0);
    sp1_zkvm::io::commit_slice(&hash);
}
