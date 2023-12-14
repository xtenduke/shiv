use serde::{Deserialize, Serialize};
use std::{fs, error::Error};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub scripts: Vec<Scripts>
}

#[derive(Serialize, Deserialize)]
pub struct Scripts {
    pub name: String,
    pub run: String
}

pub fn load_config(absolute_path: &String) -> Result<Config, Box<dyn Error>> {
    let res = match fs::read_to_string(&absolute_path) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed reading config file, does it exist? {}", &absolute_path);
            return Err(e.into());
        }
    };

    let config: Config = match serde_json::from_str(&res) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed parsing config file {}, is it valid json?", &absolute_path);
            return Err(e.into());
        }
    };

    return Ok(config);
}
