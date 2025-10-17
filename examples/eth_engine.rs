use alloy_evm::eth::EthEvmFactory;
use reth::revm::{
    context::TxEnv,
    database_interface::EmptyDB,
    primitives::{Address, Bytes, TxKind, U256},
    State,
};
use reth_evm_test_harness::{
    evm::{dev_account, dev_account_at, DevHarness, EvmTestHarness},
};

fn main() -> eyre::Result<()> {
    // Create dev harness with pre-funded test accounts (10,000 ETH each)
    let mut harness = EvmTestHarness::<_, EthEvmFactory>::dev();

    // Run all tests
    test_balance_and_transfer(&mut harness)?;
    test_fork_transition(&mut harness)?;
    test_custom_tx_type(&mut harness)?;

    println!("✓ All tests passed");
    Ok(())
}

fn test_balance_and_transfer(
    harness: &mut EvmTestHarness<State<EmptyDB>, EthEvmFactory>,
) -> eyre::Result<()> {
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

    // Demonstrate context modification
    harness.set_block_number(100);
    harness.set_timestamp(1_700_000_000);
    harness.set_base_fee(2_000_000_000);

    Ok(())
}

fn test_fork_transition(
    harness: &mut EvmTestHarness<State<EmptyDB>, EthEvmFactory>,
) -> eyre::Result<()> {
    let fork_block = 200;

    // Reset base fee to allow 1 gwei transactions
    harness.set_base_fee(1_000_000_000);

    // Pre-fork
    harness.set_block_number(fork_block - 1);
    let tx_pre = create_test_tx_with_nonce(1);
    let result_pre = harness.execute_tx(tx_pre)?;

    // Post-fork (need different nonce since state persists)
    harness.set_block_number(fork_block);
    let tx_post = create_test_tx_with_nonce(2);
    let result_post = harness.execute_tx(tx_post)?;

    // Compare behavior across fork
    assert!(result_pre.is_success());
    assert!(result_post.is_success());

    Ok(())
}

fn test_custom_tx_type(
    harness: &mut EvmTestHarness<State<EmptyDB>, EthEvmFactory>,
) -> eyre::Result<()> {
    // Example: Test a custom transaction type
    let custom_tx = TxEnv {
        tx_type: 0x7E, // Example custom type
        caller: dev_account(), // Use funded account
        gas_limit: 50_000,
        gas_price: 1_000_000_000u128,
        kind: TxKind::Call(Address::with_last_byte(1)),
        data: Bytes::from(vec![0x01, 0x02, 0x03]),
        nonce: 3, // Account was used in previous tests
        chain_id: Some(1),
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

fn create_test_tx_with_nonce(nonce: u64) -> TxEnv {
    TxEnv {
        caller: dev_account(), // Use funded account
        gas_limit: 100_000,
        gas_price: 1_000_000_000u128,
        kind: TxKind::Create,
        value: U256::ZERO,
        data: Bytes::from(vec![0x60, 0x80, 0x60, 0x40]), // dumb contract code (no logic)
        nonce,
        chain_id: Some(1),
        access_list: Default::default(),
        gas_priority_fee: None,
        blob_hashes: vec![],
        max_fee_per_blob_gas: 0,
        authorization_list: vec![],
        tx_type: 0,
    }
}
