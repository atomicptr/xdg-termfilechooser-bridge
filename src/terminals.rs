use std::{process::Command, str::FromStr};

use log::info;
use serde::{Deserialize, Serialize};
use which::which;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum Terminal {
    Alacritty,
    Foot,
    Ghostty,
    Kitty,
    Xterm,
}

fn command_name(term: &Terminal) -> Option<String> {
    match term {
        Terminal::Alacritty => String::from_str("alacritty").map_or(None, Some),
        Terminal::Ghostty => String::from_str("ghostty").map_or(None, Some),
        Terminal::Kitty => String::from_str("kitty").map_or(None, Some),
        Terminal::Xterm => String::from_str("xterm").map_or(None, Some),
        Terminal::Foot => String::from_str("foot").map_or(None, Some),
    }
}

pub fn terminal_from_env() -> Terminal {
    if let Ok(_) = which("ghostty") {
        return Terminal::Ghostty;
    }

    if let Ok(_) = which("kitty") {
        return Terminal::Kitty;
    }

    if let Ok(_) = which("alacritty") {
        return Terminal::Alacritty;
    }

    if let Ok(_) = which("foot") {
        return Terminal::Foot;
    }

    return Terminal::Xterm;
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
