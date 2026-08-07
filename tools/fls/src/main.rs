use std::process::ExitCode;

use clap::Parser;
use fls::cli::Cli;
use fls::output::{self, ErrorEnvelope};

fn main() -> ExitCode {
    let cli = Cli::parse();
    let json = cli.wants_json();
    match fls::cli::run(cli) {
        Ok(code) => ExitCode::from(code as u8),
        Err(error) => {
            if json {
                let message = error.to_string();
                let _ = output::json(&ErrorEnvelope { error: &message });
            } else {
                eprintln!("错误：{error}");
            }
            ExitCode::from(2)
        }
    }
}
