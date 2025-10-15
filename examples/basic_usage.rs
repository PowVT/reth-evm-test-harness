//! Basic usage example
//!
//! Run with: `cargo run --example basic_usage`

use alloy_evm::eth::EthEvmFactory;
use reth::revm::{
    context::TxEnv,
    primitives::{Address, Bytes, TxKind, U256},
};
use reth_chainspec::ChainSpec;
use reth_evm_test_harness::{evm::EvmTestHarness, presets};
use std::sync::Arc;

fn main() -> eyre::Result<()> {
    // Create harness with your custom EVM factory
    let chain_spec = Arc::new(ChainSpec::default());
    let mut harness = EvmTestHarness::builder()
        .with_evm_factory(EthEvmFactory::default())
        .with_chain_spec(chain_spec)
        .build();

    // Execute a basic transfer
    let tx = create_transfer_tx();
    let result = harness.execute_tx(tx)?;
    assert!(result.is_success());
    assert_eq!(result.gas_used, 21_000);

    // Test a precompile (identity)
    let identity = Address::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4]);
    let input = Bytes::from(vec![1, 2, 3, 4, 5]);
    let result = harness.execute_precompile(identity, input.clone())?;
    assert_eq!(result.output, input);

    // Change block context
    harness.set_block_number(100);
    harness.set_timestamp(1_700_000_000);
    harness.set_base_fee(2_000_000_000);

    // Use test presets
    presets::test_eip1559_transaction(&mut harness)?;
    presets::test_gas_limit(&mut harness, 21_000)?;

    println!("✓ All tests passed");
    Ok(())
}

fn create_transfer_tx() -> TxEnv {
    TxEnv {
        caller: Address::ZERO,
        gas_limit: 21_000,
        gas_price: 1_000_000_000u128,
        kind: TxKind::Call(Address::with_last_byte(1)),
        value: U256::from(1_000_000_000_000_000_000u64),
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
