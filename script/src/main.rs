//! A simple script to generate and verify the proof of a given program.

use celestia_types::{nmt::Namespace, Blob, Commitment, ExtendedHeader};
use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};
use std::fs::File;
use std::io::prelude::*;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    let my_namespace = Namespace::new_v0(&[1, 2, 3, 4, 5]).expect("Invalid namespace");

    let header_bytes = std::fs::read("header.dat").unwrap();
    let dah = ExtendedHeader::decode_and_validate(&header_bytes).unwrap();
    let row_roots = dah.dah.row_roots;

    let blob_bytes = std::fs::read("blob.dat").unwrap();
    let mut blob = Blob::new(my_namespace, blob_bytes).unwrap();
    // hardcoded because whoops
    blob.index = 8;

    let blob_size: u32 = (blob.data.len() / 512).try_into().unwrap(); // num shares
    println!("blob size: {}", blob_size);
    let square_size: u32 = row_roots.len().try_into().unwrap();
    println!("Square size: {}", square_size);
    let blob_index: u32 = blob.index.try_into().unwrap();
    let first_row_index = blob_index / square_size;
    println!("First row index: {}", first_row_index);
    let last_row_index = first_row_index + (blob_size / square_size);
    println!("last row index: {}", last_row_index);

    let proofs_file = File::open("proofs.json").unwrap();
    let proofs: Vec<celestia_types::nmt::NamespaceProof> =
        serde_json::from_reader(proofs_file).unwrap();

    let mut stdin = SP1Stdin::new();

    let shares = blob.to_shares().expect("Failed to split blob to shares");
    let leaf_hashes: Vec<_> = shares.iter().map(|share| share.as_ref()).collect();

    stdin.write(&(leaf_hashes.len() as u32));
    stdin.write(&(last_row_index as u32 - first_row_index as u32));
    stdin.write(&my_namespace);
    println!("len leaf_hashes: {}", leaf_hashes.len());
    println!("size of leaf_hash: {}", leaf_hashes[0].len());
    leaf_hashes.iter().for_each(|hash| stdin.write_slice(hash));

    println!("{}", last_row_index - first_row_index);
    for i in first_row_index..last_row_index {
        println!("i: {}", i);
        stdin.write(&row_roots[i as usize]);
        stdin.write(&proofs[i as usize]);
    }

    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");
    let result = proof.stdout.read::<bool>();
    println!("result: {}", result);
    SP1Verifier::verify(ELF, &proof).expect("verification failed");
    println!("succesfully generated and verified proof for the program!");

    /*SP1Verifier::verify(ELF, &proof).expect("verification failed");*/
    /*
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    let n = 186u32;
    stdin.write(&n);
    let mut proof = SP1Prover::prove(ELF, st0din).expect("proving failed");

    // Read output.
    let a = proof.stdout.read::<u128>();
    let b = proof.stdout.read::<u128>();
    println!("a: {}", a);
    println!("b: {}", b);

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!")
    */
}
