use std::{io, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("no config file found (look for: {searched})")]
    NotFound {searched: String},

    #[error("cannot read {}", .path.display())]
    Read {
        path: PathBuf,
        source: io::Error,
    },

    #[error("invalid config")]
    Parse(#[from] toml::de::Error),

    #[error("unsupportedSchemaVersion {found} (expected 1)")]
    UnsupportedSchemaVersion {found: u32},
}