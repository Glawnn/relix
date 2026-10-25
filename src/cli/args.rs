use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "relix",
    version,
    about = "A CLI tool to simplify software release workflows"
)]
pub(super) struct ArgsCli {
    #[command(subcommand)]
    pub(super) command: Command,
}

#[derive(Debug, Subcommand)]
pub(super) enum Command {
    /// Initialize a new release
    Init,
    /// Validate configuration and repository
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

        assert!(matches!(args.command, Command::Init));
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
    fn rejects_unknown_subcommend() {
        let result = ArgsCli::try_parse_from(["relix", "foo"]);

        assert!(result.is_err());
    }
}
