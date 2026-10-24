use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "relix",
    version,
    about = "A CLI tool to simplify software release workflows"
)]
pub(super) struct ArgsCli {}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::ArgsCli;

    #[test]
    fn verify_args_cli_definition() {
        ArgsCli::command().debug_assert();
    }
}
