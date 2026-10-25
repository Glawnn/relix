pub mod versioning;
pub mod error;

use anyhow::Ok;
use serde::Deserialize;
use versioning::VersioningStrategy;
use error::ConfigError;

const SUPPORTED_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub schema_version: u32,
    #[serde(default)]
    pub versioning: VersioningStrategy,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            schema_version: SUPPORTED_SCHEMA_VERSION,
            versioning: VersioningStrategy::default(),
        }
    }
}

impl Config {
    pub fn from_toml_str(input: &str) -> Result<Self, ConfigError> {
        let config: Config = toml::from_str(input)?;

        if config.schema_version != SUPPORTED_SCHEMA_VERSION {
            return Err(ConfigError::UnsupportedSchemaVersion { found: config.schema_version, });
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, versioning::VersioningStrategy};

    #[test]
    fn default_config_uses_default_versioning() {
        let config = Config::default();

        assert!(matches!(config.versioning, VersioningStrategy::SemVer(_)));
    }
}
