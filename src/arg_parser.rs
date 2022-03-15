use clap::{ArgEnum, Parser};
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, ArgEnum, Deserialize)]
pub enum FileType {
    Programming,
    Markup,
    Data,
    Prose,
}

/// Display programming language usage statistics for a project
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Do not use git even if the directoryis a dir repository
    #[clap(long = "no-git")]
    pub no_git: bool,

    /// Do not skip hidden/dot directories
    #[clap(long = "no-skip-dots")]
    pub no_skip_dots: bool,

    /// List of extensions to ignore.
    /// You can pass list as a single comma-separated list,
    /// or by using the flag multiple times.
    #[clap(short, long)]
    pub ignore: Option<Vec<String>>,

    /// List of file types to include in the summary.
    /// Does not support passing as comma-separated list.
    #[clap(short, long, arg_enum)]
    pub types: Option<Vec<FileType>>,

    /// Directory of the project
    #[clap(default_value_t = String::from("."))]
    pub dir: String,

    /// Maximum number of entries to show
    #[clap(short, long, default_value_t = 5)]
    pub most: usize,
}

pub trait OrDefault {
    fn types_or_default(&self) -> &[FileType];
    fn ignore_or_default(&self) -> &[String];
}

impl OrDefault for Args {
    fn types_or_default(&self) -> &[FileType] {
        if let Some(types) = self.types.as_ref() {
            types
        } else {
            Args::DEFAULT_TYPE
        }
    }
    fn ignore_or_default(&self) -> &[String] {
        if let Some(ignore) = self.ignore.as_ref() {
            ignore
        } else {
            Args::DEFAULT_IGNORE
        }
    }
}

impl Args {
    const DEFAULT_TYPE: &'static [FileType] = &[FileType::Programming];
    const DEFAULT_IGNORE: &'static [String] = &[String::new()];
}
