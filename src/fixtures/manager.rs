//! Fixture management utilities

#[cfg(feature = "fixtures")]
use super::format::{BlockFixture, TestVector};

#[cfg(feature = "fixtures")]
use crate::{Error, Result};

use std::path::{Path, PathBuf};

/// Manages loading and saving test fixtures
pub struct FixtureManager {
    fixtures_dir: PathBuf,
}

impl FixtureManager {
    /// Create a new fixture manager
    pub fn new(fixtures_dir: impl Into<PathBuf>) -> Self {
        Self {
            fixtures_dir: fixtures_dir.into(),
        }
    }

    /// Load block fixtures from a directory
    #[cfg(feature = "fixtures")]
    pub fn load_blocks(&self, name: &str) -> Result<Vec<BlockFixture>> {
        let path = self.fixtures_dir.join(name);

        if !path.exists() {
            return Err(Error::fixture(format!(
                "Fixture directory does not exist: {}",
                path.display()
            )));
        }

        let mut blocks = Vec::new();

        // Read all JSON files in the directory
        for entry in std::fs::read_dir(&path).map_err(|e| Error::fixture(e.to_string()))? {
            let entry = entry.map_err(|e| Error::fixture(e.to_string()))?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content =
                    std::fs::read_to_string(&path).map_err(|e| Error::fixture(e.to_string()))?;

                let block: BlockFixture = serde_json::from_str(&content)
                    .map_err(|e| Error::fixture(format!("Failed to parse {}: {}", path.display(), e)))?;

                blocks.push(block);
            }
        }

        // Sort blocks by number
        blocks.sort_by_key(|b| b.number);

        Ok(blocks)
    }

    /// Load a test vector
    #[cfg(feature = "fixtures")]
    pub fn load_test_vector(&self, name: &str) -> Result<TestVector> {
        let path = self.fixtures_dir.join(format!("{}.json", name));

        if !path.exists() {
            return Err(Error::fixture(format!(
                "Test vector file does not exist: {}",
                path.display()
            )));
        }

        let content = std::fs::read_to_string(&path).map_err(|e| Error::fixture(e.to_string()))?;

        serde_json::from_str(&content)
            .map_err(|e| Error::fixture(format!("Failed to parse test vector: {}", e)))
    }

    /// Save block fixtures
    #[cfg(feature = "fixtures")]
    pub fn save_blocks(&self, name: &str, blocks: &[BlockFixture]) -> Result<()> {
        let dir = self.fixtures_dir.join(name);
        std::fs::create_dir_all(&dir).map_err(|e| Error::fixture(e.to_string()))?;

        for (i, block) in blocks.iter().enumerate() {
            let path = dir.join(format!("block_{}.json", i + 1));
            let content =
                serde_json::to_string_pretty(block).map_err(|e| Error::fixture(e.to_string()))?;

            std::fs::write(&path, content).map_err(|e| Error::fixture(e.to_string()))?;
        }

        Ok(())
    }

    /// Save a test vector
    #[cfg(feature = "fixtures")]
    pub fn save_test_vector(&self, vector: &TestVector) -> Result<()> {
        let path = self.fixtures_dir.join(format!("{}.json", vector.name));

        let content =
            serde_json::to_string_pretty(vector).map_err(|e| Error::fixture(e.to_string()))?;

        std::fs::write(&path, content).map_err(|e| Error::fixture(e.to_string()))?;

        Ok(())
    }

    /// Get the fixtures directory
    pub fn fixtures_dir(&self) -> &Path {
        &self.fixtures_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixture_manager_creation() {
        let manager = FixtureManager::new("test_fixtures");
        assert_eq!(manager.fixtures_dir(), Path::new("test_fixtures"));
    }
}
