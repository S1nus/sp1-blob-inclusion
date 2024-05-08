//! A simple program to be proven inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use nmt_rs::simple_merkle::proof::Proof;
use nmt_rs::TmSha2Hasher;
use nmt_rs::{simple_merkle::proof, NamespaceId, NamespacedHash};

use celestia_types::nmt::NamespaceProof;
use celestia_types::{nmt::Namespace, Blob};
use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use std::fmt;

pub fn main() {
    // read the data root
    let mut data_root = [0u8; 32 as usize];
    sp1_zkvm::io::read_slice(&mut data_root);
    // read num rows
    let num_rows: u32 = sp1_zkvm::io::read();
    // read blob size
    let blob_size: u32 = sp1_zkvm::io::read();
    // read namespace ID
    let namespace = sp1_zkvm::io::read::<Namespace>();
    // read the row-inclusion range proof
    let range_proof: Proof<TmSha2Hasher> = sp1_zkvm::io::read();
    // read the row roots
    let mut row_roots = vec![];
    for i in 0..num_rows {
        row_roots.push(sp1_zkvm::io::read::<NamespacedHash<29>>());
    }
    // read each share of the blob
    let mut shares = vec![];
    for i in 0..blob_size {
        let mut slice = [0u8; 512];
        sp1_zkvm::io::read_slice(&mut slice);
        shares.push(slice);
    }
    // for each row spanned by the blob, we have a NMT range proof
    let mut proofs = vec![];
    for i in 0..num_rows {
        proofs.push(sp1_zkvm::io::read::<NamespaceProof>());
    }

    // We have one NMT range proof for each row spanned by the blob
    // Verify that the blob's shares go into the respective row roots
    let mut start = 0;
    for i in 0..num_rows {
        let proof = &proofs[i as usize];
        let root = &row_roots[i as usize];
        let end = start + (proof.end_idx() as usize - proof.start_idx() as usize);
        let result = proof.verify_range(&root, &shares[start..end], namespace.into());
        if result.is_err() {
            sp1_zkvm::io::write(&false);
            return;
        }
        start = end;
    }

    // Verify the row-inclusion range proof
    let blob_row_root_hashes: Vec<[u8; 32]> = row_roots.iter().map(|root| root.hash()).collect();
    let result = range_proof.verify_range(&data_root, &blob_row_root_hashes);
    if result.is_err() {
        sp1_zkvm::io::write(&false);
        return;
    }
    sp1_zkvm::io::write(&true);
}
