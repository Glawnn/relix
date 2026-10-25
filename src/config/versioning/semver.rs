use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct ConfigSemVer {
    pub initial_version: String,
}

impl Default for ConfigSemVer {
    fn default() -> Self {
        Self {
            initial_version: String::from("0.1.0"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ConfigSemVer;

    #[test]
    fn default_config_semver() {
        let config = ConfigSemVer::default();
        assert_eq!(config.initial_version, "0.1.0");
    }
}
