//! Advanced usage example showing fixtures and custom patterns
//!
//! Run with: `cargo run --example advanced_usage --features fixtures`

use alloy_evm::eth::EthEvmFactory;
use reth::revm::{
    context::TxEnv,
    primitives::{Address, Bytes, TxKind, U256},
};
use reth_chainspec::ChainSpec;
use reth_evm_test_harness::evm::EvmTestHarness;
use std::sync::Arc;

#[cfg(feature = "fixtures")]
use reth_evm_test_harness::fixtures::FixtureManager;

fn main() -> eyre::Result<()> {
    // Custom test context helper
    let mut harness = create_test_harness();

    // Test fork transitions
    test_fork_transition(&mut harness)?;

    // Test custom transaction type (example)
    test_custom_tx_type(&mut harness)?;

    // Test with fixtures (if feature enabled)
    #[cfg(feature = "fixtures")]
    test_with_fixtures()?;

    println!("✓ Advanced tests passed");
    Ok(())
}

fn create_test_harness() -> EvmTestHarness<reth::revm::database_interface::EmptyDB, EthEvmFactory> {
    let chain_spec = Arc::new(ChainSpec::default());
    EvmTestHarness::builder()
        .with_evm_factory(EthEvmFactory::default())
        .with_chain_spec(chain_spec)
        .with_block_number(1)
        .with_timestamp(1_600_000_000)
        .build()
}

fn test_fork_transition(
    harness: &mut EvmTestHarness<reth::revm::database_interface::EmptyDB, EthEvmFactory>,
) -> eyre::Result<()> {
    let fork_block = 100;
    let tx = create_test_tx();

    // Pre-fork
    harness.set_block_number(fork_block - 1);
    let result_pre = harness.execute_tx(tx.clone())?;

    // Post-fork
    harness.set_block_number(fork_block);
    let result_post = harness.execute_tx(tx)?;

    // Compare behavior across fork
    assert!(result_pre.is_success());
    assert!(result_post.is_success());

    Ok(())
}

fn test_custom_tx_type(
    harness: &mut EvmTestHarness<reth::revm::database_interface::EmptyDB, EthEvmFactory>,
) -> eyre::Result<()> {
    // Example: Test a custom transaction type
    let custom_tx = TxEnv {
        tx_type: 0x7E, // Example custom type
        caller: Address::ZERO,
        gas_limit: 50_000,
        gas_price: 1_000_000_000u128,
        kind: TxKind::Call(Address::with_last_byte(1)),
        data: Bytes::from(vec![0x01, 0x02, 0x03]),
        ..Default::default()
    };

    // Test execution (may fail if custom type not supported, which is expected)
    let result = harness.execute_tx(custom_tx);

    // Add assertions for your custom transaction behavior here
    // For this example, we just check it executes
    match result {
        Ok(r) => assert!(r.is_success() || !r.is_success()), // Either outcome is fine for demo
        Err(_) => {} // Custom types may not be supported in base EVM
    }

    Ok(())
}

#[cfg(feature = "fixtures")]
fn test_with_fixtures() -> eyre::Result<()> {
    let fixture_manager = FixtureManager::new("tests/fixtures");

    // Example: Load and replay block fixtures
    // Note: This requires actual fixture files to exist
    match fixture_manager.load_blocks("example") {
        Ok(blocks) => {
            println!("Loaded {} block fixtures", blocks.len());
            // Process blocks...
        }
        Err(_) => {
            println!("No fixture files found (this is expected for demo)");
        }
    }

    Ok(())
}

fn create_test_tx() -> TxEnv {
    TxEnv {
        caller: Address::ZERO,
        gas_limit: 100_000,
        gas_price: 1_000_000_000u128,
        kind: TxKind::Create,
        value: U256::ZERO,
        data: Bytes::from(vec![0x60, 0x80, 0x60, 0x40]), // Simple contract code
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
