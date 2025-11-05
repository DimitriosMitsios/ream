use libp2p::gossipsub::TopicHash;
use ream_consensus_lean::{block::SignedBlock, vote::SignedVote};
#[cfg(feature = "risc0")]
use ream_consensus_lean::block::BlockProof;
use ssz::Decode;
#[cfg(feature = "risc0")]
use bincode;

use super::topics::{LeanGossipTopic, LeanGossipTopicKind};
use crate::gossipsub::error::GossipsubError;

#[derive(Debug, Clone)]
pub enum LeanGossipsubMessage {
    Block(SignedBlock),
    Vote(SignedVote),
    #[cfg(feature = "risc0")]
    BlockProof(BlockProof),
}

impl LeanGossipsubMessage {
    pub fn decode(topic: &TopicHash, data: &[u8]) -> Result<Self, GossipsubError> {
        match LeanGossipTopic::from_topic_hash(topic)?.kind {
            LeanGossipTopicKind::Block => Ok(Self::Block(SignedBlock::from_ssz_bytes(data)?)),
            LeanGossipTopicKind::Vote => Ok(Self::Vote(SignedVote::from_ssz_bytes(data)?)),
            #[cfg(feature = "risc0")]
            LeanGossipTopicKind::BlockProof => {
                let (block_proof, _) = bincode::serde::decode_from_slice(data, bincode::config::standard())
                    .map_err(|e| GossipsubError::InvalidData(format!("Failed to decode BlockProof: {e}")))?;
                Ok(Self::BlockProof(block_proof))
            }
        }
    }
}
