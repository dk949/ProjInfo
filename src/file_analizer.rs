use crate::arg_parser::Args;
use crate::arg_parser::OrDefault;
use crate::file_collector::FileExtension::{Extension, Path};
use crate::language_file_extensions::Langs;
use crate::types::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Stats {
    pub langs: HashMap<String, i64>,
    pub total: usize,
}

impl Stats {
    fn new(total: usize) -> Self {
        Stats {
            langs: HashMap::new(),
            total: total,
        }
    }
}

pub fn run(args: &Args, langs: Langs, files: Files) -> Stats {
    let mut stats: Stats = Stats::new(files.len());
    let types = args.types_or_default();
    let ignore = args.ignore_or_default();
    for file in files {
        match file {
            Extension(ext) => {
                if !ignore.contains(&ext) {
                    match langs.get(ext.as_str()) {
                        Some(lang) => {
                            if types.contains(&lang.category) {
                                if stats.langs.contains_key(&lang.name) {
                                    *stats.langs.get_mut(&lang.name).expect("internal error") += 1;
                                } else {
                                    stats.langs.insert(lang.name.clone(), 1);
                                }
                            }
                        }
                        None => *stats.langs.entry("Unknown".to_string()).or_insert(0) += 1,
                    }
                }
            }
            // Note: ideally at this point I would open the file andtry to tell what it is from the contents
            Path(_) => (),
        };
    }
    return stats;
}
