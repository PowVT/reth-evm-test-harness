//! Test context that holds the test environment

use crate::Result;
use parking_lot::RwLock;
use reth::revm::primitives::{Address, B256};
use reth_chainspec::{ChainSpec, EthChainSpec};
use reth_evm::EvmFactory;
use std::sync::Arc;

/// Main test context that holds all testing state
///
/// This provides a unified interface for testing at any level (EVM, consensus, engine API, E2E).
pub struct TestContext<Evm: EvmFactory> {
    /// The EVM factory for this test
    pub evm_factory: Evm,
    /// The chain specification
    pub chain_spec: Arc<ChainSpec>,
    /// Current block environment
    pub current_block: Arc<RwLock<BlockEnv>>,
    /// Test configuration
    pub config: TestConfig,
}

/// Block environment for testing
#[derive(Debug, Clone, Default)]
pub struct BlockEnv {
    /// Current block number
    pub number: u64,
    /// Current block timestamp
    pub timestamp: u64,
    /// Block base fee
    pub base_fee: Option<u64>,
    /// Block gas limit
    pub gas_limit: u64,
    /// Block beneficiary/coinbase
    pub coinbase: Address,
    /// Previous block hash
    pub prev_randao: B256,
}

/// Test configuration options
#[derive(Debug, Clone)]
pub struct TestConfig {
    /// Enable verbose logging
    pub verbose: bool,

    /// Fail fast on first error
    pub fail_fast: bool,

    /// Maximum gas per block
    pub max_gas_limit: u64,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            fail_fast: true,
            max_gas_limit: 30_000_000,
        }
    }
}

impl<Evm: EvmFactory> TestContext<Evm> {
    /// Create a new test context
    pub fn new(evm_factory: Evm, chain_spec: Arc<ChainSpec>) -> Self {
        Self {
            evm_factory,
            chain_spec,
            current_block: Arc::new(RwLock::new(BlockEnv::default())),
            config: TestConfig::default(),
        }
    }

    /// Get the current block number
    pub fn block_number(&self) -> u64 {
        self.current_block.read().number
    }

    /// Advance to the next block
    pub fn advance_block(&self) -> Result<()> {
        let mut block = self.current_block.write();
        block.number += 1;
        block.timestamp += 12; // Default 12 second block time
        Ok(())
    }

    /// Set the current block number
    pub fn set_block_number(&self, number: u64) {
        self.current_block.write().number = number;
    }

    /// Set the current block timestamp
    pub fn set_timestamp(&self, timestamp: u64) {
        self.current_block.write().timestamp = timestamp;
    }

    /// Get the chain ID
    pub fn chain_id(&self) -> u64 {
        self.chain_spec.chain().id()
    }

    /// Enable verbose logging
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.config.verbose = verbose;
        self
    }

    /// Set fail fast mode
    pub fn with_fail_fast(mut self, fail_fast: bool) -> Self {
        self.config.fail_fast = fail_fast;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_evm::eth::EthEvmFactory;

    #[test]
    fn test_context_creation() {
        let evm_factory = EthEvmFactory::default();
        let chain_spec = Arc::new(ChainSpec::default());
        let ctx = TestContext::new(evm_factory, chain_spec);

        assert_eq!(ctx.block_number(), 0);
    }

    #[test]
    fn test_advance_block() {
        let evm_factory = EthEvmFactory::default();
        let chain_spec = Arc::new(ChainSpec::default());
        let ctx = TestContext::new(evm_factory, chain_spec);

        assert_eq!(ctx.block_number(), 0);
        ctx.advance_block().unwrap();
        assert_eq!(ctx.block_number(), 1);
    }
}
