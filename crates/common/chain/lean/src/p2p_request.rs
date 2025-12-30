use ream_consensus_lean::{block::SignedBlock, vote::SignedVote};
use ream_consensus_lean::block::BlockProof;

#[derive(Debug, Clone)]
pub enum LeanP2PRequest {
    GossipBlock(SignedBlock),
    GossipVote(SignedVote),
    GossipBlockProof(BlockProof),
}
