extern crate tdjson_sys;

use tdjson_sys::{td_set_log_file_path, td_set_log_verbosity_level};

use std::ffi::CString;
use std::error::Error;
use std::fmt;

/// This enum specifies
#[derive(Debug)]
pub enum LogError {
  TDLibError,
  OutOfRangeError,
  CStringError(std::ffi::NulError),
}

impl fmt::Display for LogError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      LogError::CStringError(err) => err.fmt(f),
      LogError::TDLibError => write!(f, "TDLib error"),
      LogError::OutOfRangeError => write!(f, "log_verbosity must be between 1 and 1024.")
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

/// Sets TDLib log file path for your application.`
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

pub fn set_log_verbosity_level(level: i32) -> Result<(), LogError> {
  if level < 1 || level > 1024 {
    Err(LogError::OutOfRangeError)
  } else {
    unsafe { td_set_log_verbosity_level(level); };
    Ok(())
  }
}
