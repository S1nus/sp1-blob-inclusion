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

    // read original data square (ODS) size
    let ods_size: u32 = sp1_zkvm::io::read();
    // read num rows
    let num_rows: u32 = sp1_zkvm::io::read();
    // read namespace ID
    let namespace = sp1_zkvm::io::read::<Namespace>();

    // read leaves
    const num_leaves: usize = 272;
    let mut leaves = [[0u8; 512]; num_leaves];
    for i in 0..num_leaves {
        sp1_zkvm::io::read_slice(&mut leaves[i]);
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
    // do middle rows

    /*for i in 0..num_rows {
        // read row root
        let root = sp1_zkvm::io::read::<NamespacedHash<29>>();
        // read proof
        let proof: celestia_types::nmt::NamespaceProof = sp1_zkvm::io::read();
        let result = proof.verify_range(
            &root,
            &leaves[..(proof.end_idx() as usize - proof.start_idx() as usize)],
            namespace.into_inner(),
        );
        if result.is_err() {
            sp1_zkvm::io::write(&false);
            return;
        }
    }*/
    //sp1_zkvm::io::write(&true);

    /*let n = sp1_zkvm::io::read::<u32>();
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut sum: u128;
    for _ in 1..n {
        sum = a + b;
        a = b;
        b = sum;
    }

    sp1_zkvm::io::write(&a);
    sp1_zkvm::io::write(&b);*/
}
