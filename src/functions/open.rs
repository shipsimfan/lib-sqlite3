use crate::SQLite3;
use std::ffi::{c_char, c_int, c_void};

#[link(name = "sqlite3")]
extern "C" {
    pub fn sqlite3_open(filename: *const c_char, db: *mut *mut SQLite3) -> c_int;
    pub fn sqlite3_open16(filename: *const c_void, db: *mut *mut SQLite3) -> c_int;
    pub fn sqlite3_open_v2(
        filename: *const c_char,
        db: *mut *mut SQLite3,
        flags: c_int,
        vfs: *const c_char,
    ) -> c_int;
}
