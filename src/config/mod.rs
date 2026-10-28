pub mod error;
pub mod versioning;

use std::path::Path;

use error::ConfigError;
use serde::Deserialize;
use versioning::VersioningStrategy;

pub const DEFAULT_TEMPLATE: &str = include_str!("default.toml");

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

const CANDIDATES: [&str; 2] = ["relix.toml", ".config/relix.toml"];

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

    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path).map_err(|source| ConfigError::Read {
            path: path.to_path_buf(),
            source,
        })?;

        Self::from_toml_str(&content)
    }

    pub fn discover(explicit: Option<&Path>) -> Result<Self, ConfigError> {
        Self::discover_in(Path::new("."), explicit)
    }

    fn discover_in(base: &Path, explicit: Option<&Path>) -> Result<Self, ConfigError> {
        if let Some(path) = explicit {
            return Self::load(path);
        }

        for candidate in CANDIDATES {
            let path = base.join(candidate);
            if path.is_file() {
                return Self::load(&path);
            }
        }

        Err(ConfigError::NotFound {
            searched: CANDIDATES.join(", "),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::{Config, ConfigError};
    use crate::config::versioning::{
        CalVerFormat, ConfigCalVer, ConfigSemVer, SemVerVersion, VersioningStrategy,
    };

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
                initial_version: SemVerVersion::try_from("1.0.0".to_string()).unwrap(),
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
            format = "YY.0M.MICRO"
        "#;

        let config = Config::from_toml_str(input).unwrap();

        assert_eq!(
            config.versioning,
            VersioningStrategy::CalVer(ConfigCalVer {
                format: CalVerFormat::try_from("YY.0M.MICRO".to_string()).unwrap()
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
    fn rejects_invalid_semver_initial_version() {
        for value in ["banane", "1.2", "1.2.3.4", "01.2.3", "1.2.3-rc.1", ""] {
            let input = format!(
                "schema_version = 1\n[versioning]\nstrategy = \"semver\"\ninitial_version = \"{value}\"\n"
            );

            let err = Config::from_toml_str(&input).unwrap_err();

            assert!(
                matches!(err, ConfigError::Parse(_)),
                "`{value}` should be rejected"
            );
        }
    }

    #[test]
    fn error_names_invalid_semver_initial_version() {
        let message = error_message(
            "schema_version = 1\n[versioning]\nstrategy = \"semver\"\ninitial_version = \"01.2.3\"",
        );

        assert!(message.contains("major cannot have leading zeros"));
    }

    #[test]
    fn rejects_invalid_calver_format() {
        for value in [
            "",
            "YYYY.FOO",
            "YYYY.PATCH",
            "M.MICRO",
            "YYYY.M.M",
            "YYYY-M",
        ] {
            let input = format!(
                "schema_version = 1\n[versioning]\nstrategy = \"calver\"\nformat = \"{value}\"\n"
            );

            let err = Config::from_toml_str(&input).unwrap_err();

            assert!(
                matches!(err, ConfigError::Parse(_)),
                "`{value}` should be rejected"
            );
        }
    }

    #[test]
    fn error_suggests_micro_instead_of_patch() {
        let message = error_message(
            "schema_version = 1\n[versioning]\nstrategy = \"calver\"\nformat = \"YYYY.PATCH\"",
        );

        assert!(message.contains("use 'MICRO'"));
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

    const SEMVER_FILE: &str = "schema_version = 1\n[versioning]\nstrategy = \"semver\"\n";
    const CALVER_FILE: &str = "schema_version = 1\n[versioning]\nstrategy = \"calver\"\n";

    #[test]
    fn discover_finds_relix_toml() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("relix.toml"), SEMVER_FILE).unwrap();

        let config = Config::discover_in(dir.path(), None).unwrap();

        assert!(matches!(config.versioning, VersioningStrategy::SemVer(_)));
    }

    #[test]
    fn discover_falls_back_to_dot_config_dir() {
        let dir = tempdir().unwrap();
        fs::create_dir(dir.path().join(".config")).unwrap();
        fs::write(dir.path().join(".config/relix.toml"), CALVER_FILE).unwrap();

        let config = Config::discover_in(dir.path(), None).unwrap();

        assert!(matches!(config.versioning, VersioningStrategy::CalVer(_)));
    }

    #[test]
    fn discover_prefers_relix_toml_over_dot_config() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("relix.toml"), SEMVER_FILE).unwrap();
        fs::create_dir(dir.path().join(".config")).unwrap();
        fs::write(dir.path().join(".config/relix.toml"), CALVER_FILE).unwrap();

        let config = Config::discover_in(dir.path(), None).unwrap();

        assert!(matches!(config.versioning, VersioningStrategy::SemVer(_)));
    }

    #[test]
    fn discover_explicit_path_wins() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("relix.toml"), SEMVER_FILE).unwrap();
        let other = dir.path().join("other.toml");
        fs::write(&other, CALVER_FILE).unwrap();

        let config = Config::discover_in(dir.path(), Some(&other)).unwrap();

        assert!(matches!(config.versioning, VersioningStrategy::CalVer(_)));
    }

    #[test]
    fn discover_fails_when_no_file() {
        let dir = tempdir().unwrap();

        let err = Config::discover_in(dir.path(), None).unwrap_err();

        assert!(matches!(err, ConfigError::NotFound { .. }));
    }

    #[test]
    fn load_fails_on_missing_file() {
        let dir = tempdir().unwrap();

        let err = Config::load(&dir.path().join("nope.toml")).unwrap_err();

        assert!(matches!(err, ConfigError::Read { .. }));
    }
}
