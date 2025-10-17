//! Dev utilities for creating test harnesses with pre-funded accounts

use super::EvmTestHarness;
use reth::revm::{
    context::TxEnv, database_interface::EmptyDB, primitives::{hardfork::SpecId, Address, KECCAK_EMPTY, U256}, state::AccountInfo, State
};
use reth_chainspec::ChainSpec;
use reth_evm::EvmFactory;
use std::sync::Arc;

/// Default test accounts with pre-funded balances (same as Anvil/Hardhat)
pub const DEV_ACCOUNTS: [&str; 10] = [
    "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
    "0x70997970C51812dc3A010C7d01b50e0d17dc79C8",
    "0x3C44CdDdB6a900fa2b585dd299e03d12fa4293BC",
    "0x90F79bf6EB2c4f870365E785982E1f101E93b906",
    "0x15d34AAf54267DB7D7c367839AAF71A00a2C6A65",
    "0x9965507D1a55bcC2695C58ba16FB37d819B0A4dc",
    "0x976EA74026E726554dB657fA54763abd0C3a0aa9",
    "0x14dC79964da2C08b23698B3D3cc7Ca32193d9955",
    "0x23618e81E3f5cdF7f54C3d65f7FBc0aBf5B21E8f",
    "0xa0Ee7A142d267C1f36714E4a8F75612F20a79720",
];

/// Default balance for dev accounts (10,000 ETH)
pub const DEV_BALANCE: u128 = 10_000_000_000_000_000_000_000u128;

/// Create a dev database with pre-funded accounts
pub fn create_dev_db() -> State<EmptyDB> {
    let mut db = State::builder().with_database(EmptyDB::default()).build();

    // Fund all dev accounts
    for account_str in &DEV_ACCOUNTS {
        let address: Address = account_str.parse().expect("valid address");
        let account_info = AccountInfo {
            balance: U256::from(DEV_BALANCE),
            nonce: 0,
            code_hash: KECCAK_EMPTY,
            code: None,
        };
        db.insert_account(address, account_info);
    }

    db
}

/// Extension trait for creating dev harnesses
pub trait DevHarness<Evm: EvmFactory> {
    /// Create a new dev harness with pre-funded test accounts
    fn dev() -> Self;

    /// Create a dev harness with a custom chain spec
    fn dev_with_chain_spec(chain_spec: Arc<ChainSpec>) -> Self;
}

impl<Evm: EvmFactory<Spec = SpecId, Tx = TxEnv> + Default>
    DevHarness<Evm> for EvmTestHarness<State<EmptyDB>, Evm>
{
    fn dev() -> Self {
        Self::dev_with_chain_spec(Arc::new(ChainSpec::default()))
    }

    fn dev_with_chain_spec(chain_spec: Arc<ChainSpec>) -> Self {
        let db = create_dev_db();
        let evm_factory = Evm::default();

        EvmTestHarness::new(evm_factory, db, chain_spec)
    }
}

/// Get the first dev account (commonly used as default sender)
pub fn dev_account() -> Address {
    DEV_ACCOUNTS[0].parse().expect("valid address")
}

/// Get dev account by index (0-9)
pub fn dev_account_at(index: usize) -> Address {
    assert!(index < DEV_ACCOUNTS.len(), "Dev account index out of bounds");
    DEV_ACCOUNTS[index].parse().expect("valid address")
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_evm::eth::EthEvmFactory;

    #[test]
    fn test_dev_db_has_funded_accounts() {
        let mut db = create_dev_db();

        for account_str in &DEV_ACCOUNTS {
            let address: Address = account_str.parse().unwrap();
            let account = db.load_cache_account(address).unwrap();
            assert_eq!(account.account_info().unwrap().balance, U256::from(DEV_BALANCE));
        }
    }

    #[test]
    fn test_dev_harness_creation() {
        let mut harness = EvmTestHarness::<State<EmptyDB>, EthEvmFactory>::dev();

        // Verify we can access the funded accounts using get_balance
        let first_account = dev_account();
        let balance = harness.get_balance(first_account).unwrap();
        assert_eq!(balance, U256::from(DEV_BALANCE));
    }
}
