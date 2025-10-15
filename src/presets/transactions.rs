//! Common transaction test scenarios

use crate::{evm::EvmTestHarness, Error, Result};
use reth::revm::{
    context::TxEnv,
    primitives::{hardfork::SpecId, Address, Bytes, TxKind, U256},
};
use reth_evm::{Database, EvmFactory};

/// Test EIP-1559 transaction execution
pub fn test_eip1559_transaction<
    DB: Database + Clone,
    Evm: EvmFactory<Spec = SpecId, Tx = TxEnv>,
>(
    harness: &mut EvmTestHarness<DB, Evm>,
) -> Result<()> {
    let tx = TxEnv {
        caller: Address::ZERO,
        gas_limit: 21_000,
        gas_price: 1_000_000_000u128, // 1 gwei
        gas_priority_fee: Some(1_000_000_000u128),
        kind: TxKind::Call(Address::with_last_byte(1)),
        value: U256::from(1_000_000_000_000_000_000u64), // 1 ETH
        data: Bytes::new(),
        nonce: 0,
        chain_id: Some(harness.chain_id()),
        access_list: Default::default(),
        blob_hashes: vec![],
        max_fee_per_blob_gas: 0,
        authorization_list: vec![],
        tx_type: 2, // EIP-1559
    };

    let result = harness.execute_tx(tx)?;

    if result.is_success() {
        Ok(())
    } else {
        Err(Error::evm_execution(
            result.revert_reason().unwrap_or("Transaction reverted"),
        ))
    }
}

/// Test that a transaction type is rejected
pub fn test_tx_rejection<DB: Database + Clone, Evm: EvmFactory<Spec = SpecId, Tx = TxEnv>>(
    harness: &mut EvmTestHarness<DB, Evm>,
    tx: TxEnv,
) -> Result<()> {
    let result = harness.execute_tx(tx);

    if result.is_err() || result.unwrap().is_revert() {
        Ok(())
    } else {
        Err(Error::evm_execution(
            "Expected transaction to be rejected, but it succeeded",
        ))
    }
}

/// Test basic value transfer
pub fn test_value_transfer<DB: Database + Clone, Evm: EvmFactory<Spec = SpecId, Tx = TxEnv>>(
    harness: &mut EvmTestHarness<DB, Evm>,
    from: Address,
    to: Address,
    value: U256,
) -> Result<()> {
    let tx = TxEnv {
        caller: from,
        gas_limit: 21_000,
        gas_price: 1_000_000_000u128,
        kind: TxKind::Call(to),
        value,
        data: Bytes::new(),
        nonce: 0,
        chain_id: Some(harness.chain_id()),
        access_list: Default::default(),
        gas_priority_fee: Default::default(),
        blob_hashes: vec![],
        max_fee_per_blob_gas: 0,
        authorization_list: vec![],
        tx_type: 0,
    };

    let result = harness.execute_tx(tx)?;

    if result.is_success() {
        Ok(())
    } else {
        Err(Error::evm_execution(
            result.revert_reason().unwrap_or("Value transfer failed"),
        ))
    }
}

/// Test contract deployment
pub fn test_contract_deployment<
    DB: Database + Clone,
    Evm: EvmFactory<Spec = SpecId, Tx = TxEnv>,
>(
    harness: &mut EvmTestHarness<DB, Evm>,
    bytecode: Bytes,
) -> Result<Address> {
    let tx = TxEnv {
        caller: Address::ZERO,
        gas_limit: 10_000_000,
        gas_price: 1_000_000_000u128,
        kind: TxKind::Create,
        value: U256::ZERO,
        data: bytecode,
        nonce: 0,
        chain_id: Some(harness.chain_id()),
        access_list: Default::default(),
        gas_priority_fee: Default::default(),
        blob_hashes: vec![],
        max_fee_per_blob_gas: 0,
        authorization_list: vec![],
        tx_type: 0,
    };

    let result = harness.execute_tx(tx)?;

    if result.is_success() {
        // TODO: Extract deployed contract address from result
        Ok(Address::ZERO)
    } else {
        Err(Error::evm_execution(
            result
                .revert_reason()
                .unwrap_or("Contract deployment failed"),
        ))
    }
}

/// Test gas limit enforcement
pub fn test_gas_limit<DB: Database + Clone, Evm: EvmFactory<Spec = SpecId, Tx = TxEnv>>(
    harness: &mut EvmTestHarness<DB, Evm>,
    gas_limit: u64,
) -> Result<()> {
    let tx = TxEnv {
        caller: Address::ZERO,
        gas_limit,
        gas_price: 1_000_000_000u128,
        kind: TxKind::Call(Address::with_last_byte(1)),
        value: U256::ZERO,
        data: Bytes::new(),
        nonce: 0,
        chain_id: Some(harness.chain_id()),
        access_list: Default::default(),
        gas_priority_fee: Default::default(),
        blob_hashes: vec![],
        max_fee_per_blob_gas: 0,
        authorization_list: vec![],
        tx_type: 0,
    };

    let result = harness.execute_tx(tx)?;

    // Check that gas used doesn't exceed gas limit
    if result.gas_used <= gas_limit {
        Ok(())
    } else {
        Err(Error::evm_execution(format!(
            "Gas used ({}) exceeded gas limit ({})",
            result.gas_used, gas_limit
        )))
    }
}
