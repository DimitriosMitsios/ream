use anyhow::{anyhow, Result};
use tokio::sync::mpsc;
use tracing::{error, info};

use ream_chain_lean::{lean_chain::LeanChainReader, p2p_request::LeanP2PRequest};
use ream_consensus_lean::{
    block::{Block, BlockBody, BlockProof, Proof, SignedBlock},
    state::LeanState,
};
use ream_network_spec::networks::lean_network_spec;
use ream_storage::tables::table::Table;
use alloy_primitives::{B256, FixedBytes};
use tree_hash::TreeHash;

use methods::{GUEST_CODE_FOR_ZK_PROOF_ELF, GUEST_CODE_FOR_ZK_PROOF_ID};
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts};

use crate::messages::LeanProverServiceMessage;

/// LeanProverService handles zero-knowledge proof generation for blocks
pub struct LeanProverService {
    /// Read-only access to the lean chain
    lean_chain: LeanChainReader,
    /// Channel for receiving messages
    receiver: mpsc::UnboundedReceiver<LeanProverServiceMessage>,
    /// Channel for sending P2P gossip requests
    outbound_gossip: mpsc::UnboundedSender<LeanP2PRequest>,
}

impl LeanProverService {
    pub fn new(
        lean_chain: LeanChainReader,
        receiver: mpsc::UnboundedReceiver<LeanProverServiceMessage>,
        outbound_gossip: mpsc::UnboundedSender<LeanP2PRequest>,
    ) -> Self {
        Self {
            lean_chain,
            receiver,
            outbound_gossip,
        }
    }

    /// Start the prover service loop
    pub async fn start(mut self) -> Result<()> {
        info!("LeanProverService started");

        loop {
            tokio::select! {
                Some(message) = self.receiver.recv() => {
                    match message {
                        LeanProverServiceMessage::GenerateBlockProof { slot } => {
                            self.handle_generate_block_proof(slot);
                        }
                    }
                }
            }
        }
    }

    /// Handle proof generation request by spawning a background task
    fn handle_generate_block_proof(&self, slot: u64) {
        info!("Spawning proof generation task for slot {}", slot);

        // Clone what we need for the background task
        let lean_chain = self.lean_chain.clone();
        let outbound_gossip = self.outbound_gossip.clone();

        // Spawn proof generation in a background task so it doesn't block
        tokio::spawn(async move {
            info!("Starting proof generation for slot {}", slot);

            match Self::generate_block_proof_task(lean_chain, slot).await {
                Ok(block_proof) => {
                    info!("Successfully generated proof for slot {}", slot);

                    // Gossip the proof
                    if let Err(err) = outbound_gossip.send(LeanP2PRequest::GossipBlockProof(block_proof)) {
                        error!("Failed to send gossip request for slot {}: {:?}", slot, err);
                    } else {
                        info!("Proof gossiped for slot {}", slot);
                    }
                }
                Err(err) => {
                    error!("Failed to generate proof for slot {}: {:?}", slot, err);
                }
            }
        });
    }

    /// Generate a zero-knowledge proof for a block (runs in background task)
    async fn generate_block_proof_task(lean_chain: LeanChainReader, slot: u64) -> Result<BlockProof> {
        // Scope the read lock to only extract necessary data, then release it immediately
        // to avoid blocking write operations (like block proposals) during proof generation
        let (head, head_state, latest_known_votes_provider) = {
            let chain = lean_chain.read().await;

            // Get the head block to use as parent
            let head = chain.head;

            let (lean_state_provider, latest_known_votes_provider) = {
                let db = chain.store.lock().await;
                (db.lean_state_provider(), db.latest_known_votes_provider())
            };

            let head_state = lean_state_provider
                .get(head)?
                .ok_or_else(|| anyhow!("Post state not found for head: {head}"))?;

            (head, head_state, latest_known_votes_provider)
        }; // READ lock is released here!

        // Create the block structure
        let mut new_block = SignedBlock {
            message: Block {
                slot,
                proposer_index: slot % lean_network_spec().num_validators,
                parent_root: head,
                state_root: B256::ZERO,
                body: BlockBody::default(),
            },
            signature: FixedBytes::default(),
        };

        // Clone state and block for proving
        let state = head_state.clone();
        let block_for_proving = new_block.clone();

        info!("Running risc0 prover for slot {} (in blocking thread)", slot);

        // Run the entire proving process in a blocking thread to avoid blocking the async runtime
        let prove_info = tokio::task::spawn_blocking(move || {
            // Setup ExecutorEnv and inject inputs to guest
            let env = ExecutorEnv::builder()
                .write(&state)
                .unwrap()
                .write(&block_for_proving)
                .unwrap()
                .build()
                .unwrap();

            let prover = default_prover();
            let opts = ProverOpts::succinct();
            prover.prove_with_opts(env, GUEST_CODE_FOR_ZK_PROOF_ELF, &opts)
        })
        .await
        .map_err(|e| anyhow!("Proof generation task panicked: {}", e))??;

        info!("Proof generation completed for slot {}", slot);

        // Extract the proven state from the receipt
        let mut state: LeanState = prove_info.receipt.journal.decode().unwrap();
        let receipt = prove_info.receipt;

        // Add attestations to the block
        loop {
            state.process_attestations(&new_block.message.body.attestations)?;
            let new_votes_to_add = latest_known_votes_provider
                .get_all_votes()?
                .into_iter()
                .filter_map(|(_, vote)| {
                    (vote.message.source == state.latest_justified
                        && !new_block.message.body.attestations.contains(&vote))
                    .then_some(vote)
                })
                .collect::<Vec<_>>();

            if new_votes_to_add.is_empty() {
                break;
            }

            for vote in new_votes_to_add {
                new_block
                    .message
                    .body
                    .attestations
                    .push(vote)
                    .map_err(|err| anyhow!("Failed to add vote to new_block: {err:?}"))?;
            }
        }

        // Update body root to account for added votes
        state.latest_block_header.body_root = new_block.message.body.tree_hash_root();

        // Compute the state root
        new_block.message.state_root = state.tree_hash_root();

        // Create the proof structure
        let proof = Proof {
            receipt,
            method_id: GUEST_CODE_FOR_ZK_PROOF_ID,
        };

        let block_proof = BlockProof {
            block: new_block.message,
            proof,
        };

        Ok(block_proof)
    }
}
