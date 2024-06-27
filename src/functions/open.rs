use crate::SQLite3;
use std::ffi::{c_char, c_int};

#[link(name = "sqlite3")]
extern "C" {
    ///
    pub fn sqlite3_open(filename: *const c_char, db: *mut *mut SQLite3) -> c_int;
}
