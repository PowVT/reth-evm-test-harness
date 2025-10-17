//! Fork transition test scenarios

use crate::{evm::EvmTestHarness, Result};
use reth::revm::{context::TxEnv, database_interface::DatabaseCommit, primitives::hardfork::SpecId};
use reth_evm::{Database, EvmFactory};

/// Test fork transition by executing transactions before and after
pub fn test_fork_transition<DB: Database + DatabaseCommit, Evm: EvmFactory<Spec = SpecId, Tx = TxEnv>>(
    harness: &mut EvmTestHarness<DB, Evm>,
    fork_block: u64,
    _pre_fork_spec: SpecId,
    _post_fork_spec: SpecId,
) -> Result<()> {
    // Test at block before fork
    harness.set_block_number(fork_block - 1);

    // TODO: Execute test transactions at pre-fork spec

    // Test at fork block
    harness.set_block_number(fork_block);

    // TODO: Execute test transactions at post-fork spec

    // Verify fork activated correctly
    Ok(())
}

/// Test that features are disabled before fork
pub fn test_feature_disabled_pre_fork<
    DB: Database + DatabaseCommit,
    Evm: EvmFactory<Spec = SpecId, Tx = TxEnv>,
>(
    harness: &mut EvmTestHarness<DB, Evm>,
    fork_block: u64,
) -> Result<()> {
    harness.set_block_number(fork_block - 1);

    // TODO: Test that fork-specific features are not available

    Ok(())
}

/// Test that features are enabled after fork
pub fn test_feature_enabled_post_fork<
    DB: Database + DatabaseCommit,
    Evm: EvmFactory<Spec = SpecId, Tx = TxEnv>,
>(
    harness: &mut EvmTestHarness<DB, Evm>,
    fork_block: u64,
) -> Result<()> {
    harness.set_block_number(fork_block + 1);

    // TODO: Test that fork-specific features are available

    Ok(())
}
