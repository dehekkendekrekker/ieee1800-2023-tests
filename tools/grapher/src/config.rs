use std::{fs, path::PathBuf};

use serde::Deserialize;
use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct Config {
    /// The name of the rule from which the graph is built.
    pub entry_point: String,
}



impl Config {
    pub fn load(path : PathBuf) -> Result<Config> {
        let contents = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}


