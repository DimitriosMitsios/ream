use ream_consensus_lean::{block::{SignedBlock, BlockProof}, vote::SignedVote};

#[derive(Debug, Clone)]
pub enum LeanP2PRequest {
    GossipBlock(SignedBlock),
    GossipVote(SignedVote),
    #[cfg(feature = "risc0")]
    GossipBlockProof(BlockProof),
}
