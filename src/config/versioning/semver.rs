use serde::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields, default)]
pub struct ConfigSemVer {
    pub initial_version: SemVerVersion,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(try_from = "String")]
pub struct SemVerVersion {
    major: u64,
    minor: u64,
    patch: u64,
}

impl Default for SemVerVersion {
    fn default() -> Self {
        Self {
            major: 0,
            minor: 1,
            patch: 0,
        }
    }
}

impl TryFrom<String> for SemVerVersion {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('.').collect();

        if parts.len() != 3 {
            return Err("expected exactly three components: MAJOR.MINOR.PATCH".into());
        }

        fn parse_components(value: &str, name: &str) -> Result<u64, String> {
            if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
                return Err(format!(
                    "{name} must conatin only digits and cannot be empty"
                ));
            }
            if value.len() > 1 && value.starts_with('0') {
                return Err(format!("{name} cannot have leading zeros"));
            }
            value
                .parse::<u64>()
                .map_err(|_| format!("{name} is too large"))
        }

        Ok(Self {
            major: parse_components(parts[0], "major")?,
            minor: parse_components(parts[1], "minor")?,
            patch: parse_components(parts[2], "patch")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{ConfigSemVer, SemVerVersion};

    #[test]
    fn default_config_semver() {
        let config = ConfigSemVer::default();
        assert_eq!(
            config.initial_version,
            SemVerVersion::try_from("0.1.0".to_string()).unwrap()
        );
    }

    #[test]
    fn parse_three_numeric_components() {
        let version = SemVerVersion::try_from("1.2.3".to_string()).unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
    }

    #[test]
    fn accept_zero_components() {
        assert!(SemVerVersion::try_from("0.0.0".to_string()).is_ok());
    }

    #[test]
    fn rejects_wrong_number_of_components() {
        for input in ["", "1.2", "1.2.3.4"] {
            assert!(SemVerVersion::try_from(input.to_string()).is_err());
        }
    }

    #[test]
    fn rejects_non_numeric_components() {
        for input in ["a.1.2", "1.x.3", "1.2.-3", "1.2.3-rc.1"] {
            assert!(SemVerVersion::try_from(input.to_string()).is_err());
        }
    }

    #[test]
    fn rejects_leading_zeroes() {
        for input in ["01.2.3", "1.02.3", "1.2.03"] {
            assert!(SemVerVersion::try_from(input.to_string()).is_err());
        }
    }

    #[test]
    fn rejects_components_larger_then_u64() {
        assert!(SemVerVersion::try_from("123456789012345678901.0.0".to_string()).is_err());
    }
}
