use crate::arg_parser::Args;
use crate::arg_parser::OrDefault;
use crate::file_collector::FileExtension::{Extension, Path};
use crate::language_file_extensions::Langs;
use crate::types::*;
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

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

trait GetMutOrInsert<K, V> {
    fn get_mut_or_insert(&mut self, k: &K, v: V) -> &mut V;
}

impl<K: Eq + Hash + Clone, V: Copy> GetMutOrInsert<K, V> for HashMap<K, V> {
    fn get_mut_or_insert(&mut self, k: &K, v: V) -> &mut V {
        if self.contains_key(k) {
            self.get_mut(k).expect("Map does not contain key")
        } else {
            self.insert(k.clone(), v);
            self.get_mut(k).expect("Map does not contain key")
        }
    }
}

pub fn run(args: &Args, langs: Langs, files: Files) -> Stats {
    let mut stats: Stats = Stats::new(files.len());
    let types = args.types_or_default();
    let ignore = args.ignore_or_default();
    let unknown = "Unknown".to_string();
    for file in files {
        match file {
            Extension(ext) => {
                if !ignore.contains(&ext) {
                    match langs.get(ext.as_str()) {
                        Some(lang) => {
                            if types.contains(&lang.category) {
                                *stats.langs.get_mut_or_insert(&lang.name, 0) += 1;
                            }
                        }
                        None => *stats.langs.get_mut_or_insert(&unknown, 0) += 1,
                    }
                }
            }
            // Note: ideally at this point I would open the file andtry to tell what it is from the contents
            Path(_) => *stats.langs.get_mut_or_insert(&unknown, 0) += 1,
        };
    }
    return stats;
}
