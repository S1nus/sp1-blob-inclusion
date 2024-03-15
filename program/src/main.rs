//! A simple program to be proven inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use nmt_rs::{simple_merkle::proof, NamespaceId, NamespacedHash};

use celestia_types::{nmt::Namespace, Blob};
use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use std::fmt;

pub fn main() {
    // NOTE: values of n larger than 186 will overflow the u128 type,
    // resulting in output that doesn't match fibonacci sequence.
    // However, the resulting proof will still be valid!

    const NUM_LEAVES: u32 = 272;
    // read num rows
    let num_rows: u32 = sp1_zkvm::io::read();
    // read namespace ID
    let namespace = sp1_zkvm::io::read::<Namespace>();

    let mut leaves = [[0u8; 512]; NUM_LEAVES as usize];
    for i in 0..NUM_LEAVES {
        sp1_zkvm::io::read_slice(&mut leaves[i as usize]);
    }

    let mut start = 0;
    for i in 0..(num_rows as usize) {
        let root = sp1_zkvm::io::read::<NamespacedHash<29>>();
        let proof: celestia_types::nmt::NamespaceProof = sp1_zkvm::io::read();
        let end = start + (proof.end_idx() as usize - proof.start_idx() as usize);
        let result = proof.verify_range(&root, &leaves[start..end], namespace.into_inner());
        println!("row {} result: {}", i, result.is_ok());
        start = end;
        if result.is_err() {
            sp1_zkvm::io::write(&false);
            return;
        }
    }

    sp1_zkvm::io::write(&true);
}
