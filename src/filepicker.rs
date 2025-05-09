use std::{path::PathBuf, str::FromStr};

use log::info;
use serde::{Deserialize, Serialize};
use which::which;

use crate::FileMode;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum FilePicker {
    Yazi,
}

fn filepicker_command(fp: FilePicker) -> String {
    match fp {
        FilePicker::Yazi => String::from_str("yazi").map_or(None, Some),
    }
    .map_or(None, |cmd| {
        which(cmd).map_or(None, |p| p.to_str().map_or(None, |s| Some(s.to_string())))
    })
    .expect(format!("unable to determine file picker command for {:?}", fp).as_str())
}

pub fn filepicker_from_env() -> FilePicker {
    if let Ok(_) = which("yazi") {
        return FilePicker::Yazi;
    }

    return FilePicker::Yazi;
}

pub fn make_filepicker_command(
    fp: FilePicker,
    mode: FileMode,
    is_directory: bool,
    _is_multiple: bool,
    write_path: &PathBuf,
    output_path: &PathBuf,
    cwd: PathBuf,
) -> String {
    let cmd = filepicker_command(fp);

    info!("determined filepicker command: {:?}", &cmd);

    match fp {
        FilePicker::Yazi => {
            let mut cmd = vec![
                cmd,
                format!(
                    "--chooser-file={}",
                    output_path
                        .to_str()
                        .expect("could not convert OsStr to &str")
                ),
            ];

            cmd.push(if is_directory && mode == FileMode::Read {
                format!(
                    "--cwd-file={}",
                    escape(
                        output_path
                            .to_str()
                            .expect("could not convert OsStr to &str")
                            .into()
                    )
                    .to_string()
                )
            } else {
                format!(
                    "--cwd-file={}",
                    escape(cwd.to_str().expect("could not convert OsStr to &str"))
                )
            });
            cmd.push(
                escape(
                    write_path
                        .to_str()
                        .expect("could not convert OsStr to &str")
                        .into(),
                )
                .to_string(),
            );

            cmd.join(" ")
        }
    }
}

fn escape(string: &str) -> String {
    shlex::try_quote(string)
        .expect("could not quote string")
        .into()
}
