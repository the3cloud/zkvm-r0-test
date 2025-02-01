use risc0_zkvm::guest::env;

fn main() {
    let input: u32 = env::read();

    let output = input + 1;

    env::commit(&output);
}
