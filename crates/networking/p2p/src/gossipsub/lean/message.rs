use libp2p::gossipsub::TopicHash;
use ream_consensus_lean::{block::SignedBlock, vote::SignedVote};
use ream_consensus_lean::block::BlockProof;
use ssz::Decode;
use bincode;

use super::topics::{LeanGossipTopic, LeanGossipTopicKind};
use crate::gossipsub::error::GossipsubError;

#[derive(Debug, Clone)]
pub enum LeanGossipsubMessage {
    Block(SignedBlock),
    Vote(SignedVote),
    BlockProof(BlockProof),
}

impl LeanGossipsubMessage {
    pub fn decode(topic: &TopicHash, data: &[u8]) -> Result<Self, GossipsubError> {
        match LeanGossipTopic::from_topic_hash(topic)?.kind {
            LeanGossipTopicKind::Block => Ok(Self::Block(SignedBlock::from_ssz_bytes(data)?)),
            LeanGossipTopicKind::Vote => Ok(Self::Vote(SignedVote::from_ssz_bytes(data)?)),
            LeanGossipTopicKind::BlockProof => {
                let block_proof: BlockProof = bincode::deserialize(data)
                    .map_err(|e| GossipsubError::InvalidData(format!("Failed to decode BlockProof: {e}")))?;
                Ok(Self::BlockProof(block_proof))
            }
        }
    }
}
