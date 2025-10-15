//! Fixture file formats

use reth::revm::primitives::{Address, Bytes, B256, U256};
use std::collections::HashMap;

#[cfg(feature = "fixtures")]
use serde::{Deserialize, Serialize};

/// A block test fixture
#[derive(Debug, Clone)]
#[cfg_attr(feature = "fixtures", derive(serde::Serialize, serde::Deserialize))]
pub struct BlockFixture {
    /// Block number
    pub number: u64,

    /// Block hash
    pub hash: B256,

    /// Parent hash
    pub parent_hash: B256,

    /// Block timestamp
    pub timestamp: u64,

    /// Gas limit
    pub gas_limit: u64,

    /// Gas used
    pub gas_used: u64,

    /// Base fee per gas
    pub base_fee_per_gas: Option<u64>,

    /// Transactions in the block (RLP encoded)
    pub transactions: Vec<Bytes>,

    /// Pre-state (optional)
    pub pre_state: Option<HashMap<Address, AccountState>>,

    /// Post-state (optional)
    pub post_state: Option<HashMap<Address, AccountState>>,
}

/// Account state for fixtures
#[derive(Debug, Clone)]
#[cfg_attr(feature = "fixtures", derive(serde::Serialize, serde::Deserialize))]
pub struct AccountState {
    /// Account balance
    pub balance: U256,

    /// Account nonce
    pub nonce: u64,

    /// Code hash
    pub code: Option<Bytes>,

    /// Storage slots
    pub storage: HashMap<U256, U256>,
}

/// A test vector containing multiple blocks
#[derive(Debug, Clone)]
#[cfg_attr(feature = "fixtures", derive(serde::Serialize, serde::Deserialize))]
pub struct TestVector {
    /// Name of the test vector
    pub name: String,

    /// Description
    pub description: Option<String>,

    /// Chain ID
    pub chain_id: u64,

    /// Genesis state
    pub genesis: HashMap<Address, AccountState>,

    /// Blocks to test
    pub blocks: Vec<BlockFixture>,
}
