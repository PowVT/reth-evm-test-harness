use reth::revm::{
    database_interface::EmptyDB,
    primitives::Address,
    State,
};
use reth_evm_test_harness::EvmTestHarness;

// Gnosis fee collector address (where base fees are minted instead of burned)
const FEE_COLLECTOR_ADDRESS: Address = Address::new([
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
    0x11, 0x11, 0x11, 0x11,
]);

fn main() -> eyre::Result<()> {
    // Create dev harness with standard EVM (for demonstration purposes)
    let mut harness = EvmTestHarness::<_, GnosisEvmFactory>::dev();

    // Test 1: Demonstrate ERC20 gas
    test_erc20_gas(&mut harness)?;

    Ok(())
}

fn test_erc20_gas(
    harness: &mut EvmTestHarness<State<EmptyDB>, GnosisEvmFactory>,
) -> eyre::Result<()> {

    Ok(())
}

