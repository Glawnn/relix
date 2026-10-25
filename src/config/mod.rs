pub mod versioning;

#[derive(Debug, Default)]
pub struct Config {
    pub versioning: versioning::ConfigVersioning,
}

#[cfg(test)]
mod tests {
    use super::{Config, versioning};

    #[test]
    fn default_config_uses_default_versioning() {
        let config = Config::default();

        assert!(matches!(
            config.versioning.strategy,
            versioning::VersioningStrategy::SemVer(_)
        ));
    }
}
