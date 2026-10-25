pub mod error;
pub mod versioning;

use error::ConfigError;
use serde::Deserialize;
use versioning::VersioningStrategy;

const SUPPORTED_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Deserialize, PartialEq)]
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
            return Err(ConfigError::UnsupportedSchemaVersion {
                found: config.schema_version,
            });
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, ConfigError};
    use crate::config::versioning::{ConfigCalVer, ConfigSemVer, VersioningStrategy};

    fn error_message(input: &str) -> String {
        let err = Config::from_toml_str(input).unwrap_err();
        format!("{:#}", anyhow::Error::from(err))
    }

    #[test]
    fn default_config_uses_default_versioning() {
        let config = Config::default();

        assert!(matches!(config.versioning, VersioningStrategy::SemVer(_)));
    }

    #[test]
    fn minimal_config_uses_defaults() {
        let config = Config::from_toml_str("schema_version = 1").unwrap();

        assert_eq!(config, Config::default());
    }

    #[test]
    fn parses_semver_with_custom_initial_version() {
        let input = r#"
            schema_version = 1

            [versioning]
            strategy = "semver"
            initial_version = "1.0.0"
        "#;

        let config = Config::from_toml_str(input).unwrap();

        assert_eq!(
            config.versioning,
            VersioningStrategy::SemVer(ConfigSemVer {
                initial_version: "1.0.0".into()
            })
        );
    }

    #[test]
    fn semver_strategy_alone_uses_default_initial_version() {
        let input = r#"
            schema_version = 1

            [versioning]
            strategy = "semver"
        "#;

        let config = Config::from_toml_str(input).unwrap();

        assert_eq!(
            config.versioning,
            VersioningStrategy::SemVer(ConfigSemVer::default())
        );
    }

    #[test]
    fn calver_strategy_alone_uses_default_format() {
        let input = r#"
            schema_version = 1

            [versioning]
            strategy = "calver"
        "#;

        let config = Config::from_toml_str(input).unwrap();

        assert_eq!(
            config.versioning,
            VersioningStrategy::CalVer(ConfigCalVer::default())
        );
    }

    #[test]
    fn parses_calver_with_custom_format() {
        let input = r#"
            schema_version = 1

            [versioning]
            strategy = "calver"
            format = "YY.MM.PATCH"
        "#;

        let config = Config::from_toml_str(input).unwrap();

        assert_eq!(
            config.versioning,
            VersioningStrategy::CalVer(ConfigCalVer {
                format: "YY.MM.PATCH".into()
            })
        );
    }

    #[test]
    fn rejects_calver_field_under_semver() {
        let input = r#"
            schema_version = 1

            [versioning]
            strategy = "semver"
            format = "YYYY.MM.PATCH"
        "#;

        let err = Config::from_toml_str(input).unwrap_err();

        assert!(matches!(err, ConfigError::Parse(_)));
    }

    #[test]
    fn rejects_unknown_strategy() {
        let input = r#"
            schema_version = 1

            [versioning]
            strategy = "foo"
        "#;

        let err = Config::from_toml_str(input).unwrap_err();

        assert!(matches!(err, ConfigError::Parse(_)));
    }

    #[test]
    fn rejects_versioning_without_strategy() {
        let input = r#"
            schema_version = 1

            [versioning]
            initial_version = "1.0.0"
        "#;

        let err = Config::from_toml_str(input).unwrap_err();

        assert!(matches!(err, ConfigError::Parse(_)));
    }

    #[test]
    fn rejects_unknown_root_field() {
        let err = Config::from_toml_str("schema_version = 1\nbanana = true").unwrap_err();

        assert!(matches!(err, ConfigError::Parse(_)));
    }

    #[test]
    fn rejects_missing_schema_version() {
        let err = Config::from_toml_str("").unwrap_err();

        assert!(matches!(err, ConfigError::Parse(_)));
    }

    #[test]
    fn rejects_unsupported_schema_version() {
        let err = Config::from_toml_str("schema_version = 2").unwrap_err();

        assert!(matches!(
            err,
            ConfigError::UnsupportedSchemaVersion { found: 2 }
        ));
    }

    #[test]
    fn rejects_malformed_toml() {
        let err = Config::from_toml_str("schema_version = ").unwrap_err();

        assert!(matches!(err, ConfigError::Parse(_)));
    }

    #[test]
    fn error_names_unknown_field_and_expected_one() {
        let message = error_message(
            "schema_version = 1\n[versioning]\nstrategy = \"semver\"\nformat = \"x\"",
        );

        assert!(message.contains("line 2"));
        assert!(message.contains("unknown field `format`"));
        assert!(message.contains("initial_version"));
    }

    #[test]
    fn error_lists_valid_strategies() {
        let message = error_message("schema_version = 1\n[versioning]\nstrategy = \"foo\"");

        assert!(message.contains("line 3"));
        assert!(message.contains("unknown variant `foo`"));
        assert!(message.contains("semver") && message.contains("calver"));
    }

    #[test]
    fn error_names_missing_strategy() {
        let message =
            error_message("schema_version = 1\n[versioning]\ninitial_version = \"1.0.0\"");

        assert!(message.contains("missing field `strategy`"));
    }

    #[test]
    fn error_names_missing_schema_version() {
        let message = error_message("");

        assert!(message.contains("missing field `schema_version`"));
    }
}
