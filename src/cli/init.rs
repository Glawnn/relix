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
        println!("created {}", path.display());
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
            _ => anyhow::Error::new(r).context(format!("cannot create {}", path.display())),
        })?;

    file.write_all(DEFAULT_TEMPLATE.as_bytes())
        .with_context(|| format!("cannot write {}", path.display()))?;
    println!("created {}", path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::run;
    use crate::config::{Config, DEFAULT_TEMPLATE};

    #[test]
    fn creates_parsable_config_in_empty_dir() {
        let dir = tempdir().unwrap();

        run(dir.path(), false).unwrap();

        let path = dir.path().join("relix.toml");
        assert!(path.is_file());
        assert_eq!(Config::load(&path).unwrap(), Config::default());
    }

    #[test]
    fn written_file_is_the_template() {
        let dir = tempdir().unwrap();

        run(dir.path(), false).unwrap();

        let content = fs::read_to_string(dir.path().join("relix.toml")).unwrap();
        assert_eq!(content, DEFAULT_TEMPLATE);
    }

    #[test]
    fn refuses_to_overwrite_existing_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("relix.toml");
        fs::write(&path, "custom = true").unwrap();

        let err = run(dir.path(), false).unwrap_err();

        assert!(err.to_string().contains("already exists"));
        assert!(err.to_string().contains("--force"));
        assert_eq!(fs::read_to_string(&path).unwrap(), "custom = true");
    }

    #[test]
    fn force_overwrites_existing_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("relix.toml");
        fs::write(&path, "custom = true").unwrap();

        run(dir.path(), true).unwrap();

        assert_eq!(fs::read_to_string(&path).unwrap(), DEFAULT_TEMPLATE);
    }

    #[test]
    fn force_creates_file_when_missing() {
        let dir = tempdir().unwrap();

        run(dir.path(), true).unwrap();

        assert!(dir.path().join("relix.toml").is_file());
    }

    #[test]
    fn fails_when_directory_does_not_exist() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope");

        let err = run(&missing, false).unwrap_err();

        assert!(format!("{err:#}").contains("cannot create"));
        assert!(!missing.exists());
    }

    #[test]
    fn does_not_touch_other_files() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("other.toml"), "keep").unwrap();

        run(dir.path(), false).unwrap();

        assert_eq!(
            fs::read_to_string(dir.path().join("other.toml")).unwrap(),
            "keep"
        );
    }
}
