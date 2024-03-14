//! A simple program to be proven inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use nmt_rs::{NamespaceId, NamespacedHash};

use celestia_types::{nmt::Namespace, Blob};
use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use std::fmt;

pub fn main() {
    // NOTE: values of n larger than 186 will overflow the u128 type,
    // resulting in output that doesn't match fibonacci sequence.
    // However, the resulting proof will still be valid!

    // Read shares
    /*let mut shares: Vec<[u8; 512]> = Vec::new();
    let mut buf = [0u8; 512];
    for _ in 0..24 {
        sp1_zkvm::io::read_slice(&mut buf);
        shares.push(buf.clone());
    };*/

    // Read blob
    //let blob: Blob = sp1_zkvm::io::read();

    // read row root
    let root = sp1_zkvm::io::read::<NamespacedHash<29>>();

    // read namespace ID
    let namespace = sp1_zkvm::io::read::<Namespace>();

    // Read proof
    let proof: celestia_types::nmt::NamespaceProof = sp1_zkvm::io::read();
    let mut leaves = [[0u8; 512]; 24];
    for i in 0..24 {
        sp1_zkvm::io::read_slice(&mut leaves[i]);
    }
    let result = proof.verify_range(&root, &leaves, namespace.into_inner());
    sp1_zkvm::io::write(&result.is_ok());

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
