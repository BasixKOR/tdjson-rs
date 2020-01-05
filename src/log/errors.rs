use std::error::Error;
use std::fmt;

/// Error when failed to perform log-related task.
#[derive(Debug)]
pub enum LogError {
  /// TDLib failes to set a new log file.
  TDLibError,
  /// Thrown if the log file path contains a zero byte.
  CStringError(std::ffi::NulError),
} // TODO Refactor this

impl fmt::Display for LogError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      LogError::CStringError(err) => err.fmt(f),
      LogError::TDLibError => write!(f, "TDLib failed to set a new log file."),
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

#[derive(Debug)]
pub struct OutOfRangeError(pub i32);

impl fmt::Display for OutOfRangeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} is not in range of 1 and 1024", self.0)
  }
}
impl Error for OutOfRangeError {}