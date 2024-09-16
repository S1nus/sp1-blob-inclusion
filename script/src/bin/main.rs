use sp1_sdk::{ProverClient, SP1Stdin};

use celestia_types::{nmt::Namespace, Blob};

const ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

fn main() {
    let my_namespace = Namespace::new_v0(&[1, 2, 3, 4, 5]).expect("Invalid namespace");

    let blob_bytes = std::fs::read("blob.dat").unwrap();
    let mut blob = Blob::new(my_namespace, blob_bytes).unwrap();
    // hardcoded because whoops
    blob.index = Some(8);

    let mut stdin = SP1Stdin::new();
    // Write the namespace to stdin
    stdin.write(&my_namespace);
    // Write the blob's data to stdin
    stdin.write_vec(blob.data);

    let client = ProverClient::new();

    let (_, report) = client.execute(ELF, stdin.clone()).run().unwrap();
    println!(
        "executed program with {} cycles",
        report.total_instruction_count()
    );

    let (pk, vk) = client.setup(ELF);
    let start_time = std::time::Instant::now();
    let mut proof = client.prove(&pk, stdin).plonk().run().unwrap();
    let elapsed_time = start_time.elapsed();
    println!("Plonk proof generation took: {:?}", elapsed_time);

    let mut blob_commitment = [0u8; 32];
    proof.public_values.read_slice(&mut blob_commitment);
    println!("blob_commitment: {:?}", blob_commitment);

    let mut hash = [0u8; 32];
    proof.public_values.read_slice(&mut hash);
    println!("hash: {:?}", hash);

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
}
