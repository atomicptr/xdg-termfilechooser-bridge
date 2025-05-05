use std::{
    fs,
    io::{Error, ErrorKind},
};

use serde::{Deserialize, Serialize};

use crate::{
    constants::APP_NAME,
    filepicker::{FilePicker, filepicker_from_env},
    terminals::{Terminal, terminal_from_env},
};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Config {
    pub terminal: Option<Terminal>,
    pub filepicker: Option<FilePicker>,
    pub start_at_last_selected_dir: Option<bool>,
}

impl Config {
    pub fn from_env() -> std::io::Result<Config> {
        Ok(Config {
            terminal: Some(terminal_from_env()),
            filepicker: Some(filepicker_from_env()),
            start_at_last_selected_dir: Some(true),
        })
    }

    pub fn load() -> std::io::Result<Config> {
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

        let mut config = Config::from_env()?;

        if config_file.exists() {
            let contents = fs::read_to_string(config_file)?;

            let other_config: Config =
                toml::from_str(&contents).expect("could not parse toml file");

            config = config.merge(&other_config);
        }

        Ok(config)
    }

    fn merge(&self, other_config: &Config) -> Config {
        let mut new_config = self.clone();

        if other_config.terminal.is_some() {
            new_config.terminal = other_config.terminal.clone();
        }

        if other_config.filepicker.is_some() {
            new_config.filepicker = other_config.filepicker.clone();
        }

        if other_config.start_at_last_selected_dir.is_some() {
            new_config.start_at_last_selected_dir = other_config.start_at_last_selected_dir;
        }

        new_config
    }
}
