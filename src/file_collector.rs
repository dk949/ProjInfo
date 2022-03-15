use crate::arg_parser::Args;
use crate::arg_parser::OrDefault;
use crate::types::ProjResult;
use std::fs::canonicalize;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
pub enum FileExtension {
    Extension(String),
    Path(String),
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

pub fn run(args: &Args) -> ProjResult<Vec<FileExtension>> {
    let dir = canonicalize(Path::new(&args.dir)).or(Err("Could not normalize path"))?;
    if dir.exists() {
        Ok(WalkDir::new(dir)
            .into_iter()
            .filter_entry(|e| {
                !args.ignore_or_default()
                    .contains(&e.file_name().to_string_lossy().to_string())
                    && (args.no_skip_dots || !is_hidden(e))
            })
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
            .map(|e| {
                let p = e.path();
                if let Some(ext) = p.extension().map(|f| f.to_string_lossy().to_string()) {
                    FileExtension::Extension(ext)
                } else {
                    FileExtension::Path(p.to_string_lossy().to_string())
                }
            })
            .collect())
    } else {
        Err("provided directory does not exist")
    }
}
