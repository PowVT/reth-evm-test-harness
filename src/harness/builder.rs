//! Builder pattern for creating test contexts

use super::context::{BlockEnv, TestConfig, TestContext};
use crate::Result;
use reth::revm::primitives::Address;
use reth_chainspec::ChainSpec;
use reth_evm::EvmFactory;
use std::sync::Arc;

/// Builder for creating `TestContext` instances
pub struct TestContextBuilder<Evm: EvmFactory> {
    evm_factory: Option<Evm>,
    chain_spec: Option<Arc<ChainSpec>>,
    block_env: BlockEnv,
    config: TestConfig,
}

impl<Evm: EvmFactory + Default> Default for TestContextBuilder<Evm> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Evm: EvmFactory + Default> TestContextBuilder<Evm> {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            evm_factory: None,
            chain_spec: None,
            block_env: BlockEnv::default(),
            config: TestConfig::default(),
        }
    }

    /// Set the EVM factory
    pub fn with_evm_factory(mut self, evm_factory: Evm) -> Self {
        self.evm_factory = Some(evm_factory);
        self
    }

    /// Set the chain specification
    pub fn with_chain_spec(mut self, chain_spec: Arc<ChainSpec>) -> Self {
        self.chain_spec = Some(chain_spec);
        self
    }

    /// Set the initial block number
    pub fn with_block_number(mut self, number: u64) -> Self {
        self.block_env.number = number;
        self
    }

    /// Set the initial block timestamp
    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.block_env.timestamp = timestamp;
        self
    }

    /// Set the block base fee
    pub fn with_base_fee(mut self, base_fee: u64) -> Self {
        self.block_env.base_fee = Some(base_fee);
        self
    }

    /// Set the block gas limit
    pub fn with_gas_limit(mut self, gas_limit: u64) -> Self {
        self.block_env.gas_limit = gas_limit;
        self
    }

    /// Set the coinbase address
    pub fn with_coinbase(mut self, coinbase: Address) -> Self {
        self.block_env.coinbase = coinbase;
        self
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

    /// Build the test context
    pub fn build(self) -> Result<TestContext<Evm>> {
        let evm_factory = self.evm_factory.unwrap_or_default();
        let chain_spec = self.chain_spec.unwrap_or_else(|| Arc::new(ChainSpec::default()));

        let mut ctx = TestContext::new(evm_factory, chain_spec);
        *ctx.current_block.write() = self.block_env;
        ctx.config = self.config;

        Ok(ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_evm::eth::EthEvmFactory;

    #[test]
    fn test_builder_defaults() {
        let ctx = TestContextBuilder::<EthEvmFactory>::new()
            .build()
            .unwrap();

        assert_eq!(ctx.block_number(), 0);
        assert!(!ctx.config.verbose);
        assert!(ctx.config.fail_fast);
    }

    #[test]
    fn test_builder_with_options() {
        let ctx = TestContextBuilder::<EthEvmFactory>::new()
            .with_block_number(100)
            .with_timestamp(1234567890)
            .with_verbose(true)
            .with_fail_fast(false)
            .build()
            .unwrap();

        assert_eq!(ctx.block_number(), 100);
        assert_eq!(ctx.current_block.read().timestamp, 1234567890);
        assert!(ctx.config.verbose);
        assert!(!ctx.config.fail_fast);
    }
}
