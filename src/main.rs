use std::{
    env,
    fs::{create_dir_all, read_to_string, remove_file, write},
    io::{Error, ErrorKind},
    path::PathBuf,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use config::load_config;
use constants::{APP_NAME, WRITE_TEXT};
use filepicker::make_filepicker_command;
use log::{error, info};
use logger::{default_logger_path, setup_logger};
use terminals::run_command;

mod config;
mod constants;
mod filepicker;
mod logger;
mod terminals;

const NUM_ARGS: usize = 5;

#[derive(Debug, PartialEq, Clone)]
enum FileMode {
    Read,
    Write,
}

fn parse_bool(value: &str) -> bool {
    value == "1"
}

fn last_selected_filepath() -> PathBuf {
    let state_dir = dirs::state_dir()
        .expect("could not determine state path")
        .join(APP_NAME);
    create_dir_all(&state_dir).expect("could not create state dir");
    state_dir.join("last-selected-path")
}

fn last_selected_path(use_default: bool) -> PathBuf {
    let last_selected_file = last_selected_filepath();

    let default_dir = dirs::download_dir()
        .or(dirs::home_dir())
        .expect("could not determine default dir");

    if use_default {
        return default_dir;
    }

    if !last_selected_file.exists() {
        return default_dir;
    }

    let path = PathBuf::from_str(
        read_to_string(last_selected_file)
            .expect("could not read last selected file")
            .as_str(),
    )
    .or::<PathBuf>(Ok(default_dir.clone()))
    .expect("could not determine last selected file");

    // make sure to only use this when it actually exists and is a dir
    if path.exists() && path.is_dir() {
        return path;
    }

    default_dir
}

// This wrapper script is invoked by xdg-desktop-portal-termfilechooser.
//
// Inputs:
// 1. "1" if multiple files can be chosen, "0" otherwise.
// 2. "1" if a directory should be chosen, "0" otherwise.
// 3. "0" if opening files was requested, "1" if writing to a file was
//    requested. For example, when uploading files in Firefox, this will be "0".
//    When saving a web page in Firefox, this will be "1".
// 4. If writing to a file, this is recommended path provided by the caller. For
//    example, when saving a web page in Firefox, this will be the recommended
//    path Firefox provided, such as "~/Downloads/webpage_title.html".
//    Note that if the path already exists, we keep appending "_" to it until we
//    get a path that does not exist.
// 5. The output path, to which results should be written.
//
// Output:
// The script should print the selected paths to the output path (argument #5),
// one path per line.
// If nothing is printed, then the operation is assumed to have been canceled.
fn main() -> std::io::Result<()> {
    setup_logger();
    let log_path = default_logger_path()?;

    let config = load_config()?;

    let args: Vec<String> = env::args().skip(1).collect();

    info!("=============================");
    info!("\tTerminal:\t\t{:?}", &config.terminal);
    info!("\tFilepicker:\t\t{:?}", &config.filepicker);
    info!("\tArguments:\t\t{:?}", &args);
    info!("\tLog file:\t\t{:?}", log_path);

    if args.len() != NUM_ARGS {
        let msg = format!(
            "Invalid number of arguments given, must be {}, {} were given.",
            NUM_ARGS,
            &args.len()
        );

        error!("{}", &msg);
        return Err(Error::new(ErrorKind::InvalidInput, msg));
    }

    let is_multiple_files = parse_bool(&args[0]);
    let is_directory = parse_bool(&args[1]);
    let file_mode = if !parse_bool(&args[2]) {
        FileMode::Read
    } else {
        FileMode::Write
    };
    let write_path = match file_mode {
        FileMode::Read => None,
        FileMode::Write => Some(PathBuf::from_str(&args[3]).expect("could not parse write path")),
    };
    let output_path = PathBuf::from_str(&args[4]).expect("could not parse output path");

    // if the write path already exists, change it by adding the unix timestamp
    let write_path = if let Some(write_path) = write_path {
        if file_mode == FileMode::Write && write_path.exists() {
            let unixts = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("unable to get timestamp");

            info!(
                "File {:?} already exists, adding timestamp {}",
                &write_path,
                &unixts.as_secs()
            );
            Some(write_path.with_file_name(format!(
                    "{}.{}.{}",
                    write_path
                        .file_stem()
                        .or(write_path.file_name())
                        .or(Some(write_path.with_file_name("_download").as_os_str()))
                        .unwrap()
                        .to_str()
                        .expect("failed to convert OsStr to String"),
                    unixts.as_secs(),
                    write_path
                        .extension()
                        .map(|s| s.to_str().expect("failed to convert OsStr to &str"))
                        .unwrap_or("unknown")
                        .to_string()
                )))
        } else {
            None
        }
    } else {
        None
    };

    // if filename starts with ".", replace it with "_" so they dont appear invisible
    let write_path = if let Some(write_path) = write_path {
        let file_name = write_path.file_name();

        if file_name.is_some() && write_path.starts_with(".") {
            let file_name = file_name
                .map(|f| {
                    f.to_str()
                        .expect("failed to convert OsStr to &str")
                        .to_string()
                })
                .map(|mut f| {
                    f.replace_range(0..1, "_");
                    f
                })
                .unwrap();
            Some(write_path.with_file_name(file_name))
        } else {
            Some(write_path)
        }
    } else {
        None
    };

    info!("\tIs multiple files?\t{:?}", &is_multiple_files);
    info!("\tIs directory?\t\t{:?}", &is_directory);
    info!("\tFile Mode:\t\t{:?}", &file_mode);
    info!("\tWrite Path:\t\t{:?}", &write_path);
    info!("\tOutput Path:\t\t{:?}", &output_path);

    let last_selected = last_selected_path(!config.start_at_last_selected_dir.unwrap_or(true));
    info!("\tLast selected path:\t{:?}", &last_selected);

    let write_path = match &file_mode {
        FileMode::Read => last_selected.clone(),
        FileMode::Write => last_selected.join(write_path.unwrap().file_name().unwrap()),
    };

    let filepicker_command = make_filepicker_command(
        config.filepicker,
        file_mode.clone(),
        is_directory,
        is_multiple_files,
        &write_path,
        &output_path,
        last_selected,
    );

    info!("filepicker command: {:?}", filepicker_command);
    run_command(config.terminal, &filepicker_command);

    info!("command exited successfully!");

    let mut output_exists = false;

    if output_path.exists() {
        let data = read_to_string(&output_path)?;
        let data = data.trim();
        output_exists = data.len() > 0;

        info!("Outfile content (selected path/s): {:?}", data);
    }

    match file_mode {
        FileMode::Read => {
            if is_directory && output_exists {
                let path = last_selected_filepath();
                write(
                    path,
                    output_path
                        .parent()
                        .expect("could not determine output_paths parent")
                        .to_str()
                        .expect("could not convert OsStr to &str"),
                )?;
            }
        }
        FileMode::Write => {
            let data = read_to_string(&write_path).unwrap_or(String::from_str("").unwrap());

            if data.as_str() == WRITE_TEXT {
                info!("Write file wasn't used so lets delete it again...");
                remove_file(write_path)?;
            }
        }
    }

    Ok(())
}
