use ream_consensus_lean::{block::{SignedBlock}, vote::SignedVote};
#[cfg(feature="risc0")]
use ream_consensus_lean::block::BlockProof;
#[derive(Debug, Clone)]
pub enum LeanP2PRequest {
    GossipBlock(SignedBlock),
    GossipVote(SignedVote),
    #[cfg(feature = "risc0")]
    GossipBlockProof(BlockProof),
}
