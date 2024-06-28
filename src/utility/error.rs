use crate::sqlite3_errstr;
use std::{
    error::Error,
    ffi::{c_int, CStr},
    fmt::{Debug, Display},
    ptr::null,
};

/// An error reported by sqlite
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SQLiteError(c_int);

impl SQLiteError {
    /// Creates a new [`SQLiteError`]
    pub const fn new(code: c_int) -> Self {
        SQLiteError(code)
    }
}

impl Error for SQLiteError {}

impl Display for SQLiteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg_ptr = unsafe { sqlite3_errstr(self.0) };
        if msg_ptr != null() {
            f.write_str(&unsafe { CStr::from_ptr(msg_ptr) }.to_string_lossy())
        } else {
            f.write_str("unknown error")
        }?;

        write!(f, " ({})", self.0)
    }
}

/// Attempts to run `expr` as an SQLite expression, converting the result code into a
/// [`Result<c_int, SQLiteError>`]
#[macro_export]
macro_rules! try_sqlite3 {
    ($expr: expr) => {{
        let result = unsafe { $expr };
        if result == $crate::SQLITE_OK {
            Ok(result)
        } else {
            Err($crate::SQLiteError::new(result))
        }
    }};
}
