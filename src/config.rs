use std::{
    fs,
    io::{Error, ErrorKind},
};

use serde::{Deserialize, Serialize};

use crate::{constants::APP_NAME, filepicker::FilePicker, terminals::Terminal};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub terminal: Terminal,
    pub filepicker: FilePicker,
    pub start_at_last_selected_dir: Option<bool>,
}

pub fn load_config() -> std::io::Result<Config> {
    let config_dir = dirs::config_dir();

    if config_dir.is_none() {
        return Err(Error::new(
            ErrorKind::NotFound,
            "Could not determine config dir",
        ));
    }

    let config_dir = config_dir.unwrap();
    let config_dir = config_dir.join(APP_NAME);

    fs::create_dir_all(&config_dir)?;

    let config_file = config_dir.join("config.toml");

    if !config_file.exists() {
        // TODO: determine from environment
        return Ok(Config::default());
    }

    let contents = fs::read_to_string(config_file)?;

    Ok(toml::from_str(&contents).expect("could not parse toml file"))
}
