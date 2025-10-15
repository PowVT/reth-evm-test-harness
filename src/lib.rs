//! # Reth EVM Test Harness
//!
//! A comprehensive testing framework for custom Reth NodeBuilder and EVM implementations.
//!
//! ## Overview
//!
//! This crate provides utilities for testing custom Ethereum execution clients built with Reth's
//! modular architecture. It supports testing at multiple levels:
//!
//! - **EVM Level**: Test custom EVM implementations in isolation
//! - **Consensus Level**: Validate consensus rules and fork transitions
//! - **Engine API Level**: Test payload building and validation without JSON-RPC
//! - **End-to-End Level**: Full node integration tests
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use reth_evm_test_harness::evm::EvmTestHarness;
//!
//! #[test]
//! fn test_custom_evm() {
//!     let harness = EvmTestHarness::builder()
//!         .with_evm_factory(MyCustomEvmFactory)
//!         .with_chainspec(my_chainspec())
//!         .build();
//!
//!     let result = harness.execute_tx(tx_env)?;
//!     assert!(result.is_ok());
//! }
//! ```
//!
//! ## Modules
//!
//! - [`harness`]: Core test context and builder abstractions
//! - [`evm`]: EVM-specific testing utilities
//! - [`engine`]: Engine API testing without network overhead
//! - [`fixtures`]: Test data and fixture management
//! - [`presets`]: Common test scenarios
//! - [`consensus`]: Consensus validation testing
//! - [`rpc`]: RPC testing utilities

pub mod consensus;
pub mod engine;
pub mod evm;
pub mod fixtures;
pub mod harness;
pub mod presets;
pub mod rpc;

// Re-export commonly used types
pub use evm::EvmTestHarness;
pub use fixtures::FixtureManager;
pub use harness::{TestContext, TestContextBuilder};

/// Common result type used throughout the harness
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during testing
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// EVM execution error
    #[error("EVM execution failed: {0}")]
    EvmExecution(String),

    /// Consensus validation error
    #[error("Consensus validation failed: {0}")]
    Consensus(String),

    /// Engine API error
    #[error("Engine API error: {0}")]
    EngineApi(String),

    /// Fixture loading/saving error
    #[error("Fixture error: {0}")]
    Fixture(String),

    /// RPC error
    #[error("RPC error: {0}")]
    Rpc(String),

    /// Generic error
    #[error("Test harness error: {0}")]
    Generic(String),

    /// Wrapped eyre error
    #[error(transparent)]
    Eyre(#[from] eyre::Report),
}

impl Error {
    /// Create a new EVM execution error
    pub fn evm_execution(msg: impl Into<String>) -> Self {
        Self::EvmExecution(msg.into())
    }

    /// Create a new consensus error
    pub fn consensus(msg: impl Into<String>) -> Self {
        Self::Consensus(msg.into())
    }

    /// Create a new engine API error
    pub fn engine_api(msg: impl Into<String>) -> Self {
        Self::EngineApi(msg.into())
    }

    /// Create a new fixture error
    pub fn fixture(msg: impl Into<String>) -> Self {
        Self::Fixture(msg.into())
    }
}
