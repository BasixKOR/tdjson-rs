extern crate tdjson_sys;

use tdjson_sys::{td_set_log_file_path, td_set_log_verbosity_level};

use std::error::Error;
use std::ffi::CString;
use std::fmt;

/// Error when failed to perform log-related task.
#[derive(Debug)]
pub enum LogError {
  /// TDLib failes to set a new log file.
  TDLibError,
  /// verbosity level is not between 1 and 1024.
  OutOfRangeError,
  /// Thrown if the log file path contains a zero byte.
  CStringError(std::ffi::NulError),
} // TODO Refactor this

impl fmt::Display for LogError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      LogError::CStringError(err) => err.fmt(f),
      LogError::TDLibError => write!(f, "TDLib failed to set a new log file."),
      LogError::OutOfRangeError => write!(f, "log_verbosity must be between 1 and 1024."),
    }
  }
}

impl Error for LogError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    if let LogError::CStringError(err) = self {
      Some(err)
    } else {
      None
    }
  }
}

impl From<std::ffi::NulError> for LogError {
  fn from(error: std::ffi::NulError) -> Self {
    LogError::CStringError(error)
  }
}

/// Sets TDLib log file path for your application.
///
/// # Errors
/// This function will return a CStringError if the supplied bytes contain an
/// internal 0 byte, or a TDLibError if TDLib returns false.
pub fn set_log_file(path: &str) -> Result<(), LogError> {
  let cpath = CString::new(path)?;
  unsafe {
    if td_set_log_file_path(cpath.as_ptr()) == 1 {
      Ok(())
    } else {
      Err(LogError::TDLibError)
    }
  }
}

/// Represents the verbosity level in TDLib.
pub enum VerbosityLevel {
  /// Corresponds to level 0 in TDLib.
  FatalErrors,
  /// Corresponds to level 1 in TDLib.
  Errors,
  /// Corresponds to level 2 in TDLib.
  Warnings,
  /// Corresponds to level 3 in TDLib.
  Information,
  /// Corresponds to level 4 in TDLib.
  Debug,
  /// Corresponds to level 5 in TDLib.
  Verbose,
  /// Uses custom value instead of predefined levels. Up to 1024 can be used to enable even more logging.
  Custom(i32),
}

fn _set_log_verbosity_level(level: i32) -> Result<(), LogError> {
  if level < 1 || level > 1024 {
    Err(LogError::OutOfRangeError)
  } else {
    unsafe { td_set_log_verbosity_level(level) };
    Ok(())
  }
}

/// Sets verbosity level of TDLib log. By default it uses  a log verbosity level of 5.
///
/// # Errors
/// This function will return an LogError::OutOfRangeError if the Custom level is not
/// between 1 and 1024.
pub fn set_log_verbosity_level(level: VerbosityLevel) -> Result<(), LogError> {
  match level {
    VerbosityLevel::FatalErrors => _set_log_verbosity_level(0),
    VerbosityLevel::Errors => _set_log_verbosity_level(1),
    VerbosityLevel::Warnings => _set_log_verbosity_level(2),
    VerbosityLevel::Information => _set_log_verbosity_level(3),
    VerbosityLevel::Debug => _set_log_verbosity_level(4),
    VerbosityLevel::Verbose => _set_log_verbosity_level(5),
    VerbosityLevel::Custom(i) => _set_log_verbosity_level(i),
  }
}
