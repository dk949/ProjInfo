use crate::types::ProjResult;
use std::convert::identity;
use std::path::Path;
use walkdir::WalkDir;

pub fn run(dir: &String) -> ProjResult<Vec<String>> {
    if Path::new(dir).exists() {
        Ok(WalkDir::new(dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
            .map(|e| {
                Some(
                    e.file_name()
                        .to_string_lossy()
                        .split('.')
                        .last()?
                        .to_string(),
                )
            })
            .filter_map(identity)
            .collect())
    } else {
        Err("provided directory does not exist".to_string())
    }
}
