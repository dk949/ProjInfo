use crate::types::ProjResult;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
pub enum FileExtension {
    Extension(String),
    Path(String),
}

pub fn run(dir: &String) -> ProjResult<Vec<FileExtension>> {
    if Path::new(dir).exists() {
        Ok(WalkDir::new(dir)
            .into_iter()
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
        Err("provided directory does not exist".to_string())
    }
}
