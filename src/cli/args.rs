use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "relix",
    version,
    about = "A CLI tool to simplify software release workflows",
    long_about = "Relix automates the repetitive parts of a release: configuration, \
                  versioning, changelog and tags.\n\n\
                  It works on a plain Git repository and behaves the same locally \
                  and in any CI.\n\n\
                  Start with `relix init` to create a relix.toml, then `relix check` \
                  to validate it.",
    after_help = "Exit codes:\n  0  success\n  1  error (message printed on stderr)\n  2  invalid command line usage"
)]
pub(super) struct ArgsCli {
    #[command(subcommand)]
    pub(super) command: Command,
}

#[derive(Debug, Subcommand)]
pub(super) enum Command {
    /// Create a relix.toml configuration file
    #[command(
        long_about = "Create a commented relix.toml in the current directory.\n\n\
                  The file is created with the default settings (SemVer). \
                  An existing relix.toml is never overwritten unless --force is given."
    )]
    Init {
        /// Overwrite an existing relix.toml
        #[arg(long)]
        force: bool,
    },
    /// Validate the configuration and the repository
    Check,
}

#[cfg(test)]
mod tests {
    use clap::{CommandFactory, Parser};

    use super::{ArgsCli, Command};

    #[test]
    fn verify_args_cli_definition() {
        ArgsCli::command().debug_assert();
    }

    #[test]
    fn parses_init_subcommand() {
        let args = ArgsCli::try_parse_from(["relix", "init"]).unwrap();

        assert!(matches!(args.command, Command::Init { .. }));
    }

    #[test]
    fn init_force_defaults_to_false() {
        let args = ArgsCli::try_parse_from(["relix", "init"]).unwrap();

        assert!(matches!(args.command, Command::Init { force: false }));
    }

    #[test]
    fn init_accepts_force_flag() {
        let args = ArgsCli::try_parse_from(["relix", "init", "--force"]).unwrap();

        assert!(matches!(args.command, Command::Init { force: true }));
    }

    #[test]
    fn init_rejects_unknown_flag() {
        let result = ArgsCli::try_parse_from(["relix", "init", "--bogus"]);

        assert!(result.is_err());
    }

    #[test]
    fn check_rejects_force_flag() {
        let result = ArgsCli::try_parse_from(["relix", "check", "--force"]);

        assert!(result.is_err());
    }

    #[test]
    fn parses_check_subcommand() {
        let args = ArgsCli::try_parse_from(["relix", "check"]).unwrap();

        assert!(matches!(args.command, Command::Check));
    }

    #[test]
    fn requires_a_subcommand() {
        let result = ArgsCli::try_parse_from(["relix"]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_unknown_subcommand() {
        let result = ArgsCli::try_parse_from(["relix", "foo"]);

        assert!(result.is_err());
    }
}
