use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::sqlite3_open;

/// Each open SQLite database is represented by a pointer to an instance of the opaque structure.
/// It is useful to think of an [`SQLite3`] pointer as an object. The [`sqlite3_open`],
/// [`sqlite3_open16`], and [`sqlite3_open_v2`] interfaces are its constructors, and
/// [`sqlite3_close`] and [`sqlite3_close_v2`] are its destructors. There are many other interfaces
/// (such as [`sqlite3_prepare_v2`], [`sqlite3_create_function`], and [`sqlite3_busy_timeout`] to
/// name but three) that are methods on an [`SQLite3`] object.
pub type SQLite3 = c_void;
