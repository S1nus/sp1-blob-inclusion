#![no_main]
sp1_zkvm::entrypoint!(main);

// This is a dummy / mock program just to output the same type values as the real program
pub fn main() {
    let commitment = [0u8; 32];
    let hash = [0u8; 32];
    sp1_zkvm::io::commit_slice(&commitment);
    sp1_zkvm::io::commit_slice(&hash);
}
