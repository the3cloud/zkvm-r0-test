use std::env;

use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    let guest_elf_path = env::var("GUEST_ELF").expect("GUEST_ELF must be set");

    let guest_elf = std::fs::read(guest_elf_path).expect("Failed to read guest ELF");

    let input: u32 = 7;
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();

    let receipt = prover.prove(env, &guest_elf).unwrap().receipt;

    let output: u32 = receipt.journal.decode().unwrap();

    println!(
        "A proof of guest execution! {} is a public output from journal ",
        output
    );
}
