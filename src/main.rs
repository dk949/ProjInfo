mod arg_parser;
mod file_analizer;
mod file_collector;
mod language_file_extensions;
mod stats_display;
mod types;

use std::process::exit;

use arg_parser::Args;
use clap::Parser;

use types::*;

fn program() -> ProjResult<()> {
    let args = Args::parse();
    let files = file_collector::run()?;
    let stats = file_analizer::run(args, files)?;
    stats_display::print(stats)?;

    Ok(())
}

fn main() {
    if let Err(err) = program() {
        eprintln!("{}", err);
        exit(1);
    } else {
        exit(0);
    }
}