use risc0_zkvm::guest::env;
use ream_consensus_lean::{
    state::LeanState,
    block::SignedBlock,
};

fn main() {
    let state: LeanState = env::read();
    let new_block: SignedBlock = env::read();
    eprintln!("{}:{}: {}", "read-signed-block", "end", env::cycle_count());
    state.state_transition(&new_block, true, false)?;
    env::commit(&state);
}
