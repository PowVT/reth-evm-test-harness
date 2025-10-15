//! EVM testing utilities for in-memory execution

mod harness;
mod result;

pub use harness::{EvmTestHarness, EvmTestHarnessBuilder};
pub use result::{ExecutionResult, EvmComparison};
