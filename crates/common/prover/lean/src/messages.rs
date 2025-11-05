/// Messages that can be sent to the Lean Prover Service
#[derive(Debug)]
pub enum LeanProverServiceMessage {
    /// Generate a proof for the block at the given slot
    GenerateBlockProof {
        slot: u64,
    },
}
