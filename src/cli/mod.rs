mod args;

use args::Command;
use clap::Parser;

pub fn run() -> anyhow::Result<()> {
    let args = args::ArgsCli::parse();

    match args.command {
        Command::Init => anyhow::bail!("Not implemented yet"),
        Command::Check => anyhow::bail!("Not implemented yet"),
    }
}
