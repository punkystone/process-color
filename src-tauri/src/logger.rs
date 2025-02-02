use std::io::Write;
use std::path::PathBuf;
use std::{
    fs::OpenOptions,
    sync::{LazyLock, Mutex},
};

use chrono::Local;

pub static LOG_FILE_PATH: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));

pub fn log(message: &str) {
    let log_file_path = LOG_FILE_PATH.lock();
    if log_file_path.is_err() {
        return;
    }
    let log_file_path = log_file_path.unwrap();

    if log_file_path.is_none() {
        return;
    }
    let log_file_path = log_file_path.as_ref().unwrap();

    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(log_file_path);
    if log_file.is_err() {
        return;
    }
    let mut log_file = log_file.unwrap();

    let _ = writeln!(
        log_file,
        "{}",
        format!(
            "[{}] - {}",
            Local::now().format("%Y-%m-%dT%H:%M:%S"),
            message
        )
    );
}

pub fn set_log_path(path: String) {
    let log_file_path = LOG_FILE_PATH.lock();
    if log_file_path.is_err() {
        return;
    }
    let mut log_file_path = log_file_path.unwrap();

    *log_file_path = Some(
        PathBuf::from(path)
            .join("log.txt")
            .to_string_lossy()
            .to_string(),
    );
}
