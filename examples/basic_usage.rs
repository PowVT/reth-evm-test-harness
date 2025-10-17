//! Basic usage example
//!
//! Run with: `cargo run --example basic_usage`

use alloy_evm::eth::EthEvmFactory;
use reth::revm::{
    context::TxEnv, primitives::{Address, Bytes, TxKind, U256}
};
use reth_evm_test_harness::{
    evm::{dev_account, dev_account_at, DevHarness, EvmTestHarness},
};

fn main() -> eyre::Result<()> {
    // Create dev harness with pre-funded test accounts (10,000 ETH each)
    let mut harness = EvmTestHarness::<_, EthEvmFactory>::dev();

    // Get initial balances
    let sender = dev_account();
    let receiver = dev_account_at(1);
    let sender_balance_before = harness.get_balance(sender)?;
    let receiver_balance_before = harness.get_balance(receiver)?;

    // Execute a basic transfer
    let tx = create_transfer_tx();
    let result = harness.execute_tx(tx)?;
    assert!(result.is_success());
    assert_eq!(result.gas_used, 21_000);

    // Verify balance changes
    let sender_balance_after = harness.get_balance(sender)?;
    let receiver_balance_after = harness.get_balance(receiver)?;

    // Calculate expected balances
    let transfer_amount = U256::from(1_000_000_000_000_000_000u64); // 1 ETH
    let gas_cost = U256::from(21_000) * U256::from(1_000_000_000u128); // gas_used * gas_price
    assert_eq!(sender_balance_after, sender_balance_before - transfer_amount - gas_cost);
    assert_eq!(receiver_balance_after, receiver_balance_before + transfer_amount);

    // Test a precompile (identity at 0x04) using a different account to avoid nonce issues
    let identity = Address::with_last_byte(4); // Identity precompile
    let input = Bytes::from(vec![1, 2, 3, 4, 5]);
    let result = harness.execute_precompile(identity, input.clone(), Some(receiver))?;
    assert_eq!(result.output, input);

    // Change block context and demonstrate context modification
    harness.set_block_number(100);
    harness.set_timestamp(1_700_000_000);
    harness.set_base_fee(2_000_000_000);

    Ok(())
}

fn create_transfer_tx() -> TxEnv {
    TxEnv {
        caller: dev_account(), // Use first pre-funded dev account
        gas_limit: 21_000,
        gas_price: 1_000_000_000u128,
        kind: TxKind::Call(dev_account_at(1)), // Send to second dev account
        value: U256::from(1_000_000_000_000_000_000u64), // 1 ETH
        data: Bytes::new(),
        nonce: 0,
        chain_id: Some(1),
        access_list: Default::default(),
        gas_priority_fee: None,
        blob_hashes: vec![],
        max_fee_per_blob_gas: 0,
        authorization_list: vec![],
        tx_type: 0,
    }
}
