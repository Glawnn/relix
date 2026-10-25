pub mod versioning;

use versioning::VersioningStrategy;

const SUPPORTED_SCHEMA_VERSION: u32 = 1;

#[derive(Debug)]
pub struct Config {
    pub schema_version: u32,
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

#[cfg(test)]
mod tests {
    use super::{Config, versioning::VersioningStrategy};

    #[test]
    fn default_config_uses_default_versioning() {
        let config = Config::default();

        assert!(matches!(config.versioning, VersioningStrategy::SemVer(_)));
    }
}
