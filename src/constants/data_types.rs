use std::ffi::c_int;

/// A 64-bit signed integer
pub const SQLITE_INTEGER: c_int = 1;

/// A 64-bit IEEE floating point number
pub const SQLITE_FLOAT: c_int = 2;

/// String
pub const SQLITE_TEXT: c_int = 3;

/// BLOB
pub const SQLITE_BLOB: c_int = 4;

/// NULL
pub const SQLITE_NULL: c_int = 5;
