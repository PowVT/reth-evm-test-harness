//! EVM execution result types

use reth::revm::primitives::{Bytes, Log};

/// Result of EVM execution
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    /// Whether execution succeeded
    pub success: bool,
    /// Gas used during execution
    pub gas_used: u64,
    /// Gas refunded
    pub gas_refunded: u64,
    /// Output data
    pub output: Bytes,
    /// Logs emitted
    pub logs: Vec<Log>,
    /// Revert reason if execution failed
    pub revert_reason: Option<String>,
}

impl ExecutionResult {
    /// Create a successful execution result
    pub fn success(gas_used: u64, output: Bytes) -> Self {
        Self {
            success: true,
            gas_used,
            gas_refunded: 0,
            output,
            logs: Vec::new(),
            revert_reason: None,
        }
    }

    /// Create a failed execution result
    pub fn revert(gas_used: u64, reason: impl Into<String>) -> Self {
        Self {
            success: false,
            gas_used,
            gas_refunded: 0,
            output: Bytes::new(),
            logs: Vec::new(),
            revert_reason: Some(reason.into()),
        }
    }

    /// Check if execution succeeded
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// Check if execution reverted
    pub fn is_revert(&self) -> bool {
        !self.success
    }

    /// Get the revert reason if execution failed
    pub fn revert_reason(&self) -> Option<&str> {
        self.revert_reason.as_deref()
    }

    /// Add a log to the execution result
    pub fn with_log(mut self, log: Log) -> Self {
        self.logs.push(log);
        self
    }

    /// Set gas refunded
    pub fn with_gas_refunded(mut self, gas_refunded: u64) -> Self {
        self.gas_refunded = gas_refunded;
        self
    }
}

/// Comparison between two EVM execution results
#[derive(Debug, Clone)]
pub struct EvmComparison {
    /// The test EVM result
    pub test_result: ExecutionResult,
    /// The reference EVM result
    pub reference_result: ExecutionResult,
    /// Whether the results match
    pub matches: bool,
    /// Differences found
    pub differences: Vec<String>,
}

impl EvmComparison {
    /// Create a new comparison
    pub fn new(test_result: ExecutionResult, reference_result: ExecutionResult) -> Self {
        let mut differences = Vec::new();
        let mut matches = true;

        if test_result.success != reference_result.success {
            differences.push(format!(
                "Execution status differs: test={}, reference={}",
                test_result.success, reference_result.success
            ));
            matches = false;
        }

        if test_result.gas_used != reference_result.gas_used {
            differences.push(format!(
                "Gas used differs: test={}, reference={}",
                test_result.gas_used, reference_result.gas_used
            ));
            matches = false;
        }

        if test_result.output != reference_result.output {
            differences.push(format!(
                "Output differs: test={}, reference={}",
                test_result.output, reference_result.output
            ));
            matches = false;
        }

        if test_result.logs.len() != reference_result.logs.len() {
            differences.push(format!(
                "Log count differs: test={}, reference={}",
                test_result.logs.len(),
                reference_result.logs.len()
            ));
            matches = false;
        }

        Self {
            test_result,
            reference_result,
            matches,
            differences,
        }
    }

    /// Check if the results match
    pub fn is_match(&self) -> bool {
        self.matches
    }

    /// Get the differences
    pub fn differences(&self) -> &[String] {
        &self.differences
    }

    /// Assert that results match, panicking with details if they don't
    pub fn assert_match(&self) {
        if !self.matches {
            panic!(
                "EVM execution results differ:\n{}",
                self.differences.join("\n")
            );
        }
    }
}
