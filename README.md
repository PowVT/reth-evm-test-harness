# reth-evm-test-harness

A testing framework for custom [Reth](https://github.com/paradigmxyz/reth) EVM implementations.

## Why?

Testing custom Ethereum execution clients is hard. Most projects use bash scripts that spin up Docker containers and make JSON-RPC calls—slow, flaky, and difficult to debug.

Reth Node Builder SDK users need test tooling which provides fast, deterministic, type-safe Rust without requiring full node setup or network calls.

## Key features
- **EVM Testing**: Execute transactions and test precompiles in-memory
- **Block Context**: Set block number, timestamp, and base fee for fork testing
- **Test Fixtures**: Load and replay block vectors from JSON
- **Test Presets**: Common EIP compliance tests (EIP-1559, gas limits, etc.)

## Installation

```bash
[dev-dependencies]
# Import with entire feature set
reth-evm-test-harness = { git = "https://github.com/powvt/reth-evm-test-harness", features = ["fixtures", "engine"] }
```

## Quick Start

The harness provides a builder API to create test contexts for your custom EVM implementation:

```rust
use reth_evm_test_harness::evm::EvmTestHarness;

let mut harness = EvmTestHarness::builder()
    .with_evm_factory(YourCustomEvmFactory::default())
    .with_chain_spec(your_chainspec())
    .build();
```

## Examples

See the [`examples/`](examples/) directory:

```bash
cargo run --example basic_usage
cargo run --example advanced_usage --features fixtures
```

## Contributing

Contributions welcome, please open an issue or PR to start contributing!

## License

MIT OR Apache-2.0
