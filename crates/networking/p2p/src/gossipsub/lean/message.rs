use libp2p::gossipsub::TopicHash;
use ream_consensus_lean::{block::SignedBlock, vote::SignedVote};
#[cfg(feature = "risc0")]
use ream_consensus_lean::block::BlockProof;
#[cfg(feature = "risc0")]
use bincode;
use ssz::Decode;

use super::topics::{LeanGossipTopic, LeanGossipTopicKind};
use crate::gossipsub::error::GossipsubError;

#[cfg(not(feature = "risc0"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeanGossipsubMessage {
    Block(SignedBlock),
    Vote(SignedVote),
}

#[cfg(feature = "risc0")]
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
            #[cfg(feature = "risc0")]
            LeanGossipTopicKind::BlockProof => {
                // Use bincode for BlockProof since it contains risc0 Receipt
                let block_proof = bincode::serde::decode_from_slice(data, bincode::config::standard()).map(|(v, _)| v)
                    .map_err(|e| GossipsubError::InvalidTopic(format!("Failed to deserialize BlockProof: {e}")))?;
                Ok(Self::BlockProof(block_proof))
            }
        }
    }
}
