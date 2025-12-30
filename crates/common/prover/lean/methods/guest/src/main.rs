#![no_main]

use ere_platform_sp1::{sp1_zkvm, Platform, SP1Platform};
use ream_consensus_lean::{block::SignedBlock, state::LeanState};

sp1_zkvm::entrypoint!(main);

type P = SP1Platform;

fn main() {
    // Read whole input provided by host (prefixed by ERE)
    let input = P::read_whole_input();

    // Deserialize LeanState first, then SignedBlock from the same byte slice
    let mut cursor = std::io::Cursor::new(&input);
    let mut state: LeanState =
        bincode::deserialize_from(&mut cursor).expect("failed to read LeanState");
    let new_block: SignedBlock =
        bincode::deserialize_from(&mut cursor).expect("failed to read SignedBlock");

    // Execute state transition
    state.state_transition(&new_block, true, false);

    // Commit the result (serialize and write as public output)
    let output = bincode::serialize(&state).expect("failed to serialize state");
    P::write_whole_output(&output);
}
