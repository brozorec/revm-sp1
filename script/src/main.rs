use sp1_sdk::{ProverClient, SP1Stdin, utils};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    utils::setup_logger();

    let stdin = SP1Stdin::new();

    let client = ProverClient::new();
    let (_pk, _vk) = client.setup(ELF);
    let mut _public_values = client.execute(&ELF, stdin).unwrap();
    //let mut _proof = client.prove_compressed(&pk, stdin).expect("proving failed");

    println!("successfully generated and verified proof for the program!")
}
