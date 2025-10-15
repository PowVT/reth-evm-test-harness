//! Engine API test harness

use crate::{Error, Result};
use reth::revm::primitives::B256;
use reth_evm::EvmFactory;

#[cfg(feature = "engine")]
use reth_payload_primitives::PayloadBuilderAttributes;

/// Engine API test harness for testing payload building and validation
///
/// This allows testing the Engine API without JSON-RPC overhead.
pub struct EngineApiTestHarness<Evm: EvmFactory> {
    #[allow(dead_code)]
    evm_factory: Evm,
}

impl<Evm: EvmFactory> EngineApiTestHarness<Evm> {
    /// Create a new Engine API test harness
    pub fn new(evm_factory: Evm) -> Self {
        Self { evm_factory }
    }

    /// Build a payload with the given attributes
    #[cfg(feature = "engine")]
    pub async fn build_payload<Attrs: PayloadBuilderAttributes>(
        &mut self,
        _attrs: Attrs,
    ) -> Result<B256> {
        // TODO: Implement payload building
        // This would use the EVM factory to build a block
        Err(Error::engine_api("Not yet implemented"))
    }

    /// Validate a payload
    pub async fn validate_payload(&mut self, _payload: Vec<u8>) -> Result<PayloadStatus> {
        // TODO: Implement payload validation
        // This would validate the payload against the EVM
        Err(Error::engine_api("Not yet implemented"))
    }

    /// Update the forkchoice
    pub async fn update_forkchoice(&mut self, _update: ForkchoiceUpdate) -> Result<()> {
        // TODO: Implement forkchoice updates
        Err(Error::engine_api("Not yet implemented"))
    }
}

/// Payload validation status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PayloadStatus {
    /// Payload is valid
    Valid,
    /// Payload is invalid
    Invalid,
    /// Payload validation is pending
    Syncing,
}

/// Forkchoice update
#[derive(Debug, Clone)]
pub struct ForkchoiceUpdate {
    /// Head block hash
    pub head_block_hash: B256,
    /// Safe block hash
    pub safe_block_hash: B256,
    /// Finalized block hash
    pub finalized_block_hash: B256,
}
