//! A simple script to generate and verify the proof of a given program.

use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};
use celestia_types::{Blob, nmt::Namespace, Commitment, ExtendedHeader};
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
    blob.index = 8;

    let blob_size: usize = (blob.data.len()/512).try_into().unwrap(); // num shares
    println!("blob size: {}", blob_size);
    let square_size: usize = row_roots.len().try_into().unwrap();
    println!("Square size: {}", square_size);
    let blob_index: usize = blob.index.try_into().unwrap();
    let first_row_index =  blob_index / square_size;
    println!("First row index: {}", first_row_index);
    let last_row_index = first_row_index + (blob_size / square_size);
    println!("last row index: {}", last_row_index);

    let proofs_file = File::open("proofs.json").unwrap();
    let proofs: Vec<celestia_types::nmt::NamespaceProof> = serde_json::from_reader(proofs_file).unwrap();

    let mut stdin = SP1Stdin::new();
    stdin.write(&row_roots[0]);
    stdin.write(&my_namespace);
    for i in 0..256 {
        stdin.write_slice(blob.data[i*512..(i+1)*512].as_ref());
    }
    stdin.write(&proofs[0]);
    let mut proof = SP1Prover::execute(ELF, stdin).expect("execution failed");
    let result = proof.read::<bool>();
    println!("result: {}", result);
    /*SP1Verifier::verify(ELF, &proof).expect("verification failed");*/
    /*
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    let n = 186u32;
    stdin.write(&n);
    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

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
