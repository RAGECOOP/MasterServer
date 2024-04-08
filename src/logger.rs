//! THIS IS JUST A SIMPLE CONSOLE LOGGER!

use chrono::{NaiveDate, NaiveTime, Utc};
use colored::*;
use std::{
  fs,
  fs::File,
  io::prelude::*,
  path::Path,
  sync::{Mutex, MutexGuard},
};

static mut LOG_FILE: Mutex<Option<File>> = Mutex::new(None);
fn _log_file_callback<F>(callback: F)
where
  F: Fn(&mut MutexGuard<Option<File>>),
{
  if let Ok(mut r) = unsafe { LOG_FILE.lock() } {
    callback(&mut r)
  }
}

fn _create_log_file(time: &NaiveTime, date: &NaiveDate) {
  _log_file_callback(|file| {
    // we already have a file
    if file.is_some() {
      return;
    }

    // create a "logs" folder in the current directory if none with that name exists
    let log_path = Path::new(crate::get_current_dir()).join("logs");
    fs::create_dir_all(&log_path).expect("couldn't create `logs` dir");

    // create a path to the desired file
    let path = log_path.join(format!("{}_{}.log", date, time.format("%H-%M-%S")));

    // open the path in write-only mode
    match File::create(&path) {
      Ok(r) => file.replace(r),
      Err(e) => panic!("couldn't create {}: {}", path.display(), e),
    };
  });
}

fn _write_log_file(text: &str) {
  _log_file_callback(|file| {
    // we don't have a file to write anything in
    if file.is_none() {
      return;
    }

    let t = file.as_mut().unwrap();
    writeln!(t, "{}", text).ok();
  });
}

/// Write something to our console and log file.
pub fn log<T: Into<String>>(state: &str, text: T) {
  let now = Utc::now();
  let time = now.time();
  let date = now.date_naive();

  _create_log_file(&time, &date);

  let s = match state {
    "warning" => "WARNING".bright_yellow(),
    "error" => "ERROR".bright_red(),
    "creating" | "loading" | "successfully" | "starting" => state.to_uppercase().bright_green(),
    _ => "INFO".bright_white(),
  };

  let t = text.into();
  let log_time = format!("[{} {}]", date, time.format("%H:%M:%S"));
  println!("{} {} {}", log_time, s.bold(), t);

  _write_log_file(&format!("{} {} {}", log_time, state.to_uppercase(), t));
}

#[macro_export]
macro_rules! log {
  ($t:literal, $m:expr) => {
    crate::logger::log($t, $m)
  }
}