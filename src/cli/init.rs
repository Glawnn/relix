use std::{
    fs::{self, OpenOptions},
    io::{ErrorKind, Write},
    path::Path,
};

use anyhow::Context;

use crate::config::DEFAULT_TEMPLATE;

pub(super) fn run(dir: &Path, force: bool) -> anyhow::Result<()> {
    let path = dir.join("relix.toml");

    if force {
        fs::write(&path, DEFAULT_TEMPLATE)
            .with_context(|| format!("cannot write {}", path.display()))?;
        return Ok(());
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(|r| match r.kind() {
            ErrorKind::AlreadyExists => anyhow::anyhow!(
                "{} already exists (use --force to overwrite)",
                path.display()
            ),
            _ => anyhow::Error::new(r).context(format!("cannot creat {}", path.display())),
        })?;

    file.write_all(DEFAULT_TEMPLATE.as_bytes())
        .with_context(|| format!("cannot write {}", path.display()))
}
