use celestia_types::hash::Hash;
use celestia_types::nmt::NamespacedHashExt;
use celestia_types::{nmt::Namespace, Blob, Commitment, ExtendedHeader};
use sp1_sdk::{utils, ProverClient, SP1Stdin};
use std::fs::File;
use std::io::prelude::*;

use nmt_rs::simple_merkle::db::MemDb;
use nmt_rs::simple_merkle::tree::MerkleTree;
use nmt_rs::{NamespacedHash, TmSha2Hasher};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    let my_namespace = Namespace::new_v0(&[1, 2, 3, 4, 5]).expect("Invalid namespace");
    let header_bytes = std::fs::read("header.dat").unwrap();
    let dah = ExtendedHeader::decode_and_validate(&header_bytes).unwrap();

    let eds_row_roots = &dah.dah.row_roots();
    let eds_column_roots = &dah.dah.column_roots();
    let data_tree_leaves: Vec<_> = eds_row_roots
        .iter()
        .chain(eds_column_roots.iter())
        .map(|root| root.to_array())
        .collect();

    // "Data root" is the merkle root of the EDS row and column roots
    let hasher = TmSha2Hasher {}; // Tendermint Sha2 hasher
    let mut tree: MerkleTree<MemDb<[u8; 32]>, TmSha2Hasher> = MerkleTree::with_hasher(hasher);
    for leaf in data_tree_leaves {
        tree.push_raw_leaf(&leaf);
    }
    // Ensure that the data root is the merkle root of the EDS row and column roots
    assert_eq!(dah.dah.hash(), Hash::Sha256(tree.root()));

    // extended data square (EDS) size
    let eds_size = eds_row_roots.len();
    // original data square (ODS) size
    let ods_size = eds_size / 2;

    let blob_bytes = std::fs::read("blob.dat").unwrap();
    let mut blob = Blob::new(my_namespace, blob_bytes).unwrap();
    // hardcoded because whoops
    blob.index = Some(8);

    let shares = blob.to_shares().expect("Failed to split blob to shares");
    let share_values: Vec<_> = shares.iter().map(|share| share.as_ref()).collect();

    let blob_index: usize = blob.index.unwrap().try_into().unwrap();
    let blob_size: usize = blob.data.len() / 512;
    let first_row_index: usize = blob_index / ods_size;
    let last_row_index: usize = first_row_index + (blob_size / ods_size);

    let proofs_file = File::open("proofs.json").unwrap();
    // NMT range proofs, from leaves into row roots.
    let proofs: Vec<celestia_types::nmt::NamespaceProof> =
        serde_json::from_reader(proofs_file).unwrap();
    // For each row spanned by the blob, you should have one NMT range proof into a row root.
    assert_eq!(proofs.len(), last_row_index + 1 - first_row_index);

    let rp = tree.build_range_proof(first_row_index..last_row_index + 1);

    let mut stdin = SP1Stdin::new();
    // write the DA header
    stdin.write_slice(&dah.dah.hash().as_bytes());
    // write "num rows" spanned by the blob
    stdin.write(&(last_row_index as u32 - first_row_index as u32));
    // write num shares
    stdin.write(&(blob_size as u32));
    // write namespace
    stdin.write(&my_namespace);
    // write the range proof
    stdin.write(&rp);
    // write the row roots
    for row_root in eds_row_roots[first_row_index..last_row_index + 1].iter() {
        stdin.write(&row_root);
    }
    // write the shares
    for s in share_values {
        stdin.write_slice(s);
    }

    // write the proofs {
    for proof in proofs {
        stdin.write(&proof);
    }

    let prover_client = ProverClient::new();
    let mut public_values = prover_client.execute(&ELF, stdin).unwrap();
    println!("gnerated proof");
    let result = public_values.read::<bool>();
    println!("result: {}", result);
}
