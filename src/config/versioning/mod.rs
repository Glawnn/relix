mod calver;
mod semver;

use serde::Deserialize;

pub use calver::ConfigCalVer;
pub use semver::ConfigSemVer;

#[derive(Debug, Deserialize)]
#[serde(tag = "strategy", rename_all = "lowercase")]
pub enum VersioningStrategy {
    SemVer(ConfigSemVer),
    CalVer(ConfigCalVer),
}

impl Default for VersioningStrategy {
    fn default() -> Self {
        Self::SemVer(ConfigSemVer::default())
    }
}

#[cfg(test)]
mod tests {
    use super::VersioningStrategy;

    #[test]
    fn default_strategy_is_semver() {
        assert!(matches!(
            VersioningStrategy::default(),
            VersioningStrategy::SemVer(_)
        ));
    }
}
