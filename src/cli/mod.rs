mod args;

use clap::Parser;

pub fn run() {
    let _args_cli = args::ArgsCli::parse();
}
