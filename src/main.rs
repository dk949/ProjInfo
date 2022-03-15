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
    let langs = language_file_extensions::get()?;
    let files = file_collector::run(&args)?;
    let stats = file_analizer::run(&args, langs, files);
    stats_display::print(&args, stats)?;

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
