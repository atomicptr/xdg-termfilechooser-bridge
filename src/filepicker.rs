use std::{fs::write, path::PathBuf, str::FromStr};

use log::info;
use serde::{Deserialize, Serialize};
use which::which;

use crate::{FileMode, constants::WRITE_TEXT};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum FilePicker {
    #[default]
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
            let cwd_param = format!(
                "--cwd-file={}",
                cwd.to_str().expect("could not convert OsStr to &str")
            );

            match mode {
                FileMode::Write => {
                    write(&write_path, WRITE_TEXT).expect("could not create write file");
                    cmd.push(cwd_param);
                    cmd.push(
                        write_path
                            .to_str()
                            .expect("could not convert OsStr to &str")
                            .to_string(),
                    );

                    cmd.join(" ")
                }
                FileMode::Read => {
                    cmd.push(if is_directory {
                        format!(
                            "--cwd-file={}",
                            output_path
                                .to_str()
                                .expect("could not convert OsStr to &str")
                                .to_string()
                        )
                    } else {
                        cwd_param
                    });
                    cmd.push(
                        write_path
                            .to_str()
                            .expect("could not convert OsStr to &str")
                            .to_string(),
                    );

                    cmd.join(" ")
                }
            }
        }
    }
}
