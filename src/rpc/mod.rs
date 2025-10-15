//! RPC testing utilities

/// Placeholder for RPC testing
///
/// This module will contain utilities for testing RPC endpoints
/// without requiring a full network setup.
pub struct TestRpcClient;

impl TestRpcClient {
    /// Create a new test RPC client
    pub fn new() -> Self {
        Self
    }
}

impl Default for TestRpcClient {
    fn default() -> Self {
        Self::new()
    }
}
