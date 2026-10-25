mod calver;
mod semver;

pub use calver::ConfigCalVer;
pub use semver::ConfigSemVer;

#[derive(Debug)]
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
