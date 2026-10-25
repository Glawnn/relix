use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields, default)]
pub struct ConfigCalVer {
    pub format: String,
}

impl Default for ConfigCalVer {
    fn default() -> Self {
        Self {
            format: String::from("YYYY.MM.PATCH"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ConfigCalVer;

    #[test]
    fn default_config_calver() {
        let config = ConfigCalVer::default();
        assert_eq!(config.format, "YYYY.MM.PATCH");
    }
}
