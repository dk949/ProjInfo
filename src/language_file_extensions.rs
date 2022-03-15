use crate::arg_parser::FileType;
use crate::types::*;
use dirs::config_dir;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Lang {
    pub name: String,
    pub category: FileType,
}

pub type Langs = HashMap<String, Lang>;

pub fn get() -> ProjResult<Langs> {
    let config = config_dir()
        .ok_or("could not find config directory")?
        .join("projinfo")
        .join("langs.json");
    if !config.exists() {
        Err("file projinfo/langs.json, which should have been created during installaiton, is missing")
    } else {
        let file = fs::read_to_string(config).or(Err("Could not read langs.json"))?;
        serde_json::from_str::<HashMap<String, Lang>>(&file)
            .or(Err("Could not deserialise projinfo/langs.json"))
    }
}
