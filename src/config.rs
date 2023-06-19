use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub scripts: Vec<Scripts>
}

#[derive(Serialize, Deserialize)]
pub struct Scripts {
    pub name: String,
    pub run: String
}

pub fn load_config(absolute_path: &String) -> Config {
    let res = match fs::read_to_string(&absolute_path) {
        Ok(res) => res,
        Err(e) => panic!("Failed to read config file: {}: {}", &absolute_path, e),
    };

    let config: Config = match serde_json::from_str(&res) {
        Ok(config) => config,
        Err(e) => panic!("Failed to load config: {}", e)
    };

    return config;
}
