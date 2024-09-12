mod cmd;
mod error;
mod toml;
mod config_man;

use std::env;
use std::io::{self, Write};
use std::process::ExitCode;
use std::fs;

use clap::Parser;

use crate::cmd::{Cmd, Execute};
use crate::error::SilentExit;
use crate::config_man::{get_config_path, create_config};

const POSTHOG_API_KEY: Option<&str> = option_env!("POSTHOG_API_KEY");

#[tokio::main]
pub async fn main() -> ExitCode {
    // Forcibly disable backtraces.
    env::remove_var("RUST_LIB_BACKTRACE");
    env::remove_var("RUST_BACKTRACE");

    match POSTHOG_API_KEY {
        Some(key) => println!("POSTHOG_API_KEY is set"),
        None => eprintln!("Warning: POSTHOG_API_KEY is not set"),
    }

    let flag_file = get_config_path().unwrap().with_file_name(".vpm_welcome_shown");
    if !flag_file.exists() {
        create_config().unwrap();

        println!("Welcome to vpm!");
        println!("We collect anonymous usage data to improve the tool.");
        println!("The following information will be collected:");
        println!(" - The version of vpm you are using");
        println!(" - Which commands you run and when (not including arguments, input, or output)");
        println!("No personal information will be collected.");
        println!("To opt-out, run `vpm config --analytics false`. You may change this at any time.\n");
        println!("Rerun your command to accept and continue.");

        fs::write(flag_file, "").unwrap();
        return ExitCode::SUCCESS;
    }

    match Cmd::parse().execute().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => match e.downcast::<SilentExit>() {
            Ok(SilentExit { code }) => code.into(),
            Err(e) => {
                _ = writeln!(io::stderr(), "vpm: {e:?}");
                ExitCode::FAILURE
            }
        },
    }
}
