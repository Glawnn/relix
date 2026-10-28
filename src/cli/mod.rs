mod args;
mod init;
use args::Command;
use clap::Parser;
use std::path::Path;
pub fn run() -> anyhow::Result<()> {
    let args = args::ArgsCli::parse();

    match args.command {
        Command::Init { force } => init::run(Path::new("."), force),
        Command::Check => anyhow::bail!("Not implemented yet"),
    }
}
