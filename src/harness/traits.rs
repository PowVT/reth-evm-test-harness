//! Core traits for testable components

use reth_chainspec::{ChainSpec, EthChainSpec};

#[cfg(feature = "engine")]
use reth_node_api::FullNodeComponents;

/// Trait for nodes that can be tested with this harness
#[cfg(feature = "engine")]
pub trait TestNode: FullNodeComponents + Clone + Send + Sync + 'static {
    /// The chain spec type for this node
    type ChainSpec: TestableChainSpec;

    /// Create a node instance for testing
    fn test_instance() -> Self;
}

/// Trait for chain specs that can be used in tests
pub trait TestableChainSpec: EthChainSpec + Clone + Send + Sync + 'static {
    /// Create a test instance of this chain spec
    fn test_spec() -> Self;
}

impl TestableChainSpec for ChainSpec {
    fn test_spec() -> Self {
        // Return a basic mainnet-like spec for testing
        ChainSpec::default()
    }
}
