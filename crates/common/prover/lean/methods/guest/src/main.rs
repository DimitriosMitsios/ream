#![no_main]

use bincode;
use ere_platform_sp1::{sp1_zkvm, Platform, SP1Platform};
use serde::Deserialize;

sp1_zkvm::entrypoint!(main);

type P = SP1Platform;

use ream_consensus_lean::{block::SignedBlock, state::LeanState};

fn main() {
    let mut state: LeanState = env::read();
    let new_block: SignedBlock = env::read();

    // Read whole input provided by host (prefixed by ere)
    let input = P::read_whole_input();

    // Deserialize LeanState first, then SignedBlock from the same byte slice.
    // Use bincode::deserialize_from on a cursor to read sequentially.
    let mut cursor = std::io::Cursor::new(&input);

    let mut state: LeanState =
        bincode::deserialize_from(&mut cursor).expect("failed to read LeanState");
    let new_block: SignedBlock =
        bincode::deserialize_from(&mut cursor).expect("failed to read SignedBlock");

    state.state_transition(&new_block, true, false);
    env::commit(&state);
}
