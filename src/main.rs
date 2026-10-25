use std::process::ExitCode;

fn main() -> ExitCode {
    match relix::cli::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            println!("error: {e:#}");
            ExitCode::FAILURE
        }
    }
}
