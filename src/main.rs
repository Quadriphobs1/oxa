mod error;
mod oxa;
mod reporter;
mod scanner;
mod token;

use crate::error::{exit_with_return_code, ErrorCode};
use crate::oxa::Oxa;
use std::env;

fn main() {
    if cfg!(debug_assertions) {
        setup_logger(log::LevelFilter::Debug)
    } else {
        setup_logger(log::LevelFilter::Info)
    }

    let mut oxa = Oxa::new();

    let args: Vec<String> = env::args().collect();
    match args.len() {
        i if i > 2 => {
            println!("Usage: oxa [script]");
        }
        2 => {
            log::info!("Starting with a file");
            exit_with_return_code(oxa.run_file(&args[1]));
        }
        _ => {
            log::info!("Starting with prompt");
            exit_with_return_code(oxa.run_prompt());
        }
    }
}

fn setup_logger(level: log::LevelFilter) {
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .filter(None, level)
        .init()
}
