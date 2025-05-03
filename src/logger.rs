use std::{
    fs::create_dir_all,
    io::{self, Error, ErrorKind},
    path::PathBuf,
};

use fern::colors::{Color, ColoredLevelConfig};

use crate::constants::APP_NAME;

fn cache_dir() -> std::io::Result<PathBuf> {
    let cache_dir = dirs::cache_dir();

    if cache_dir.is_none() {
        return Err(Error::new(
            ErrorKind::NotFound,
            "Could not determine log dir",
        ));
    }

    let cache_dir = cache_dir.unwrap();
    let cache_dir = cache_dir.join(APP_NAME);

    create_dir_all(&cache_dir)?;

    Ok(cache_dir)
}

pub fn default_logger_path() -> std::io::Result<PathBuf> {
    let cache_dir = cache_dir()?;
    Ok(cache_dir.join("log"))
}

pub fn setup_logger() {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%H:%S"),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(io::stdout())
        .chain(
            fern::log_file(default_logger_path().expect("could not determine log file"))
                .expect("could not determine log file"),
        )
        .apply()
        .expect("could not create logger");
}
