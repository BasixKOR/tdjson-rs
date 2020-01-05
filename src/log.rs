extern crate tdjson_sys;

use tdjson_sys::{td_set_log_file_path, td_set_log_verbosity_level};

use std::ffi::CString;
use std::error::Error;
use std::fmt;

/// This enum specifies
#[derive(Debug)]
pub enum LogFileError {
  TDLibError,
  CStringError(std::ffi::NulError),
}

impl fmt::Display for LogFileError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let LogFileError::CStringError(err) = self {
      err.fmt(f)
    } else {
      write!(f, "TDLib error")
    }
  }
}

impl Error for LogFileError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    if let LogFileError::CStringError(err) = self {
      Some(err)
    } else {
      None
    }
  }
}

impl From<std::ffi::NulError> for LogFileError {
  fn from(error: std::ffi::NulError) -> Self {
    LogFileError::CStringError(error)
  }
}

/// Sets TDLib log file path for your application.`
pub fn set_log_file(path: &str) -> Result<(), LogFileError> {
  let cpath = CString::new(path)?;
  unsafe {
    if td_set_log_file_path(cpath.as_ptr()) == 1 {
      Ok(())
    } else {
      Err(LogFileError::TDLibError)
    }
  }
}

pub fn set_log_verbosity_level(level: i32) {
  unsafe {
    td_set_log_verbosity_level(level);
  }
}
