mod calver;
mod semver;

pub use calver::ConfigCalVer;
pub use semver::ConfigSemVer;

#[derive(Debug)]
pub enum VersioningStrategy {
    SemVer(ConfigSemVer),
    CalVer(ConfigCalVer),
}

#[derive(Debug)]
pub struct ConfigVersioning {
    pub strategy: VersioningStrategy,
}

impl Default for ConfigVersioning {
    fn default() -> Self {
        Self {
            strategy: VersioningStrategy::SemVer(ConfigSemVer::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ConfigVersioning, VersioningStrategy};

    #[test]
    fn default_strategy_is_semver() {
        let config = ConfigVersioning::default();
        assert!(matches!(config.strategy, VersioningStrategy::SemVer(_)));
    }
}
