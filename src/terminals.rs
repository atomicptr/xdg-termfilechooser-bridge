use std::{process::Command, str::FromStr};

use log::info;
use serde::{Deserialize, Serialize};
use which::which;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Terminal {
    #[default]
    Ghostty,
    Kitty,
}

fn command_name(term: &Terminal) -> Option<String> {
    match term {
        Terminal::Ghostty => String::from_str("ghostty").map_or(None, Some),
        Terminal::Kitty => String::from_str("kitty").map_or(None, Some),
    }
}

pub fn run_command(term: Terminal, run_cmd: &str) {
    let command = command_name(&term)
        .map_or(None, |command| which(command).map_or(None, Some))
        .expect(format!("could not determine command for terminal: {:?}", &term).as_str());

    info!("determined terminal command: {:?}", &command);

    Command::new(command)
        .arg("-e")
        .arg(run_cmd)
        .output()
        .expect(format!("failed to execute command {:?}", run_cmd).as_str());
}
