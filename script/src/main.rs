//! A simple script to generate and verify the proof of a given program.

use celestia_types::{nmt::Namespace, Blob, Commitment, ExtendedHeader};
use nmt_rs::NamespacedHash;
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
    blob.index = 8;

    let blob_size: usize = (blob.data.len() / 512).try_into().unwrap(); // num shares
    println!("blob size: {}", blob_size);
    let square_size: usize = row_roots.len().try_into().unwrap();
    println!("Square size: {}", square_size);
    let blob_index: usize = blob.index.try_into().unwrap();
    let first_row_index = blob_index / square_size;
    println!("First row index: {}", first_row_index);
    let last_row_index = first_row_index + (blob_size / square_size);
    println!("last row index: {}", last_row_index);

    let proofs_file = File::open("proofs.json").unwrap();
    let proofs: Vec<celestia_types::nmt::NamespaceProof> =
        serde_json::from_reader(proofs_file).unwrap();

    let bytes = bincode::serialize(&row_roots[0]).unwrap();
    let t: NamespacedHash<29> = bincode::deserialize(&bytes).unwrap();
    println!("{:?}", t);

    let mut stdin = SP1Stdin::new();
    stdin.write(&row_roots[0]);
    stdin.write(&my_namespace);
    stdin.write(&proofs[0]);

    /*let mut shares = [[0u8; 512]; 24];
    for i in 0..24 {
        shares[i] = blob.data[i * 512..(i + 1) * 512].try_into().unwrap();
        stdin.write_slice(blob.data[i * 512..(i + 1) * 512].as_ref());
    }*/

    let shares = blob.to_shares().expect("Failed to split blob to shares");
    let leaf_hashes: Vec<_> = shares.iter().map(|share| share.as_ref()).collect();
    for i in 0..24 {
        println!("{:?}", leaf_hashes[i].len());
        stdin.write_slice(leaf_hashes[i]);
    }

    /*for i in 0..256 {
        stdin.write_slice(blob.data[i*512..(i+1)*512].as_ref());
    }*/

    let result =
        proofs[0].verify_range(&row_roots[0], &leaf_hashes[..24], my_namespace.into_inner());
    println!("result: {}", result.is_ok());

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
