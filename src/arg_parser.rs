#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum FileType {
    Programming,
    Markup,
    Data,
    Prose,
}

use clap::{ArgEnum, Parser};

/// Display programming language usage statistics for a project
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Do not use gitignroe
    #[clap(short = 'n', long = "no-ignore")]
    no_gitignore: bool,

    /// List of files, extensions or directories to ignore.
    /// You can pass list as a single comma-separated list,
    /// or by using the flag multiple times.
    #[clap(short, long)]
    ignore: Option<Vec<String>>,

    /// List of file types to include in the summary.
    /// Does not support passing as comma-separated list.
    #[clap(short, long, arg_enum)]
    types: Option<Vec<FileType>>,
}
