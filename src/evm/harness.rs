//! Core EVM test harness for in-memory execution

use super::result::HarnessExecutionResult;
use crate::{Error, Result};
use reth::revm::{
    context::{BlockEnv, CfgEnv, TxEnv},
    database_interface::EmptyDB,
    primitives::{hardfork::SpecId, Address, Bytes, TxKind, U256},
};
use reth_chainspec::{ChainSpec, EthChainSpec};
use reth_evm::{Database, Evm, EvmEnv, EvmFactory};
use std::sync::Arc;

/// In-memory EVM test harness
///
/// This allows testing EVM execution without a full node or database.
pub struct EvmTestHarness<DB: Database, Evm: EvmFactory> {
    /// The EVM factory
    evm_factory: Evm,
    /// The database
    db: DB,
    /// The chain specification
    chain_spec: Arc<ChainSpec>,
    /// Current block environment
    block_env: BlockEnv,
    /// EVM configuration
    cfg_env: CfgEnv,
}

impl<Evm: EvmFactory<Spec = SpecId, Tx = TxEnv> + Default> EvmTestHarness<EmptyDB, Evm> {
    /// Create a new builder
    pub fn builder() -> EvmTestHarnessBuilder<EmptyDB, Evm> {
        EvmTestHarnessBuilder::new()
    }
}

impl<DB: Database + Clone, Evm: EvmFactory<Spec = SpecId, Tx = TxEnv>> EvmTestHarness<DB, Evm> {
    /// Create a new EVM test harness
    pub fn new(evm_factory: Evm, db: DB, chain_spec: Arc<ChainSpec>) -> Self {
        let mut cfg_env = CfgEnv::default();
        cfg_env.chain_id = chain_spec.chain().id();
        cfg_env.spec = SpecId::CANCUN;

        Self {
            evm_factory,
            db,
            chain_spec,
            block_env: BlockEnv::default(),
            cfg_env,
        }
    }

    /// Execute a transaction
    pub fn execute_tx(&mut self, tx: TxEnv) -> Result<HarnessExecutionResult> {
        let db = self.db.clone();
        let env = EvmEnv {
            block_env: self.block_env.clone(),
            cfg_env: self.cfg_env.clone(),
        };

        let mut evm = self.evm_factory.create_evm(db, env);

        match evm.transact(tx) {
            Ok(result_and_state) => {
                let result = result_and_state.result;

                match result {
                    reth::revm::context_interface::result::ExecutionResult::Success {
                        output,
                        gas_used,
                        gas_refunded,
                        ..
                    } => {
                        Ok(HarnessExecutionResult {
                            success: true,
                            gas_used,
                            gas_refunded,
                            output: output.into_data(),
                            logs: Vec::new(), // TODO: Extract logs from state
                            revert_reason: None,
                        })
                    }
                    reth::revm::context_interface::result::ExecutionResult::Revert {
                        output,
                        gas_used,
                    } => {
                        Ok(HarnessExecutionResult {
                            success: false,
                            gas_used,
                            gas_refunded: 0,
                            output,
                            logs: Vec::new(),
                            revert_reason: Some("Transaction reverted".to_string()),
                        })
                    }
                    reth::revm::context_interface::result::ExecutionResult::Halt {
                        reason,
                        gas_used,
                    } => {
                        Ok(HarnessExecutionResult {
                            success: false,
                            gas_used,
                            gas_refunded: 0,
                            output: Bytes::new(),
                            logs: Vec::new(),
                            revert_reason: Some(format!("Transaction halted: {:?}", reason)),
                        })
                    }
                }
            }
            Err(e) => Err(Error::evm_execution(format!(
                "EVM execution failed: {:?}",
                e
            ))),
        }
    }

    /// Execute a precompile call
    pub fn execute_precompile(
        &mut self,
        address: Address,
        input: Bytes,
    ) -> Result<HarnessExecutionResult> {
        let tx = TxEnv {
            caller: Address::ZERO,
            gas_limit: 10_000_000,
            gas_price: 1_000_000_000u128, // 1 gwei
            kind: TxKind::Call(address),
            value: U256::ZERO,
            data: input,
            nonce: 0,
            chain_id: Some(self.chain_spec.chain().id()),
            access_list: Default::default(),
            gas_priority_fee: Default::default(),
            blob_hashes: vec![],
            max_fee_per_blob_gas: 0,
            authorization_list: vec![],
            tx_type: 0,
        };

        self.execute_tx(tx)
    }

    /// Set the block number
    pub fn set_block_number(&mut self, number: u64) {
        self.block_env.number = U256::from(number);
    }

    /// Set the block timestamp
    pub fn set_timestamp(&mut self, timestamp: u64) {
        self.block_env.timestamp = U256::from(timestamp);
    }

    /// Set the block base fee
    pub fn set_base_fee(&mut self, base_fee: u64) {
        self.block_env.basefee = base_fee;
    }

    /// Get the current block number
    pub fn block_number(&self) -> u64 {
        self.block_env.number.to()
    }

    /// Get the chain ID
    pub fn chain_id(&self) -> u64 {
        self.cfg_env.chain_id
    }

    /// Get a reference to the database
    pub fn db(&self) -> &DB {
        &self.db
    }

    /// Get a mutable reference to the database
    pub fn db_mut(&mut self) -> &mut DB {
        &mut self.db
    }
}

/// Builder for `EvmTestHarness`
pub struct EvmTestHarnessBuilder<DB: Database, Evm: EvmFactory> {
    evm_factory: Option<Evm>,
    db: Option<DB>,
    chain_spec: Option<Arc<ChainSpec>>,
    block_number: u64,
    timestamp: u64,
    base_fee: Option<u64>,
    spec_id: SpecId,
}

impl<DB: Database + Default + Clone, Evm: EvmFactory<Spec = SpecId, Tx = TxEnv> + Default> Default
    for EvmTestHarnessBuilder<DB, Evm>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<DB: Database + Default + Clone, Evm: EvmFactory<Spec = SpecId, Tx = TxEnv> + Default>
    EvmTestHarnessBuilder<DB, Evm>
{
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            evm_factory: None,
            db: None,
            chain_spec: None,
            block_number: 0,
            timestamp: 0,
            base_fee: None,
            spec_id: SpecId::CANCUN,
        }
    }

    /// Set the EVM factory
    pub fn with_evm_factory(mut self, evm_factory: Evm) -> Self {
        self.evm_factory = Some(evm_factory);
        self
    }

    /// Set the database
    pub fn with_db(mut self, db: DB) -> Self {
        self.db = Some(db);
        self
    }

    /// Set the chain specification
    pub fn with_chain_spec(mut self, chain_spec: Arc<ChainSpec>) -> Self {
        self.chain_spec = Some(chain_spec);
        self
    }

    /// Set the initial block number
    pub fn with_block_number(mut self, block_number: u64) -> Self {
        self.block_number = block_number;
        self
    }

    /// Set the initial timestamp
    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// Set the base fee
    pub fn with_base_fee(mut self, base_fee: u64) -> Self {
        self.base_fee = Some(base_fee);
        self
    }

    /// Set the spec ID
    pub fn with_spec_id(mut self, spec_id: SpecId) -> Self {
        self.spec_id = spec_id;
        self
    }

    /// Build the harness
    pub fn build(self) -> EvmTestHarness<DB, Evm> {
        let evm_factory = self.evm_factory.unwrap_or_default();
        let db = self.db.unwrap_or_default();
        let chain_spec = self
            .chain_spec
            .unwrap_or_else(|| Arc::new(ChainSpec::default()));

        let mut harness = EvmTestHarness::new(evm_factory, db, chain_spec);
        harness.set_block_number(self.block_number);
        harness.set_timestamp(self.timestamp);
        if let Some(base_fee) = self.base_fee {
            harness.set_base_fee(base_fee);
        }
        harness.cfg_env.spec = self.spec_id;

        harness
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_evm::eth::EthEvmFactory;

    #[test]
    fn test_harness_creation() {
        let harness = EvmTestHarness::<EmptyDB, EthEvmFactory>::builder().build();

        assert_eq!(harness.block_number(), 0);
        assert!(harness.chain_id() > 0);
    }

    #[test]
    fn test_set_block_env() {
        let harness = EvmTestHarness::<EmptyDB, EthEvmFactory>::builder()
            .with_block_number(100)
            .with_timestamp(1234567890)
            .with_base_fee(1_000_000_000)
            .build();

        assert_eq!(harness.block_number(), 100);
        assert_eq!(harness.block_env.timestamp, U256::from(1234567890));
        assert_eq!(harness.block_env.basefee, 1_000_000_000);
    }
}
