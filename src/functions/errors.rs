use crate::SQLite3;
use std::ffi::{c_char, c_int, c_void};

#[link(name = "sqlite3")]
extern "C" {
    pub fn sqlite3_errcode(db: *mut SQLite3) -> c_int;
    pub fn sqlite3_extended_errcode(db: *mut SQLite3) -> c_int;
    pub fn sqlite3_errmsg(db: *mut SQLite3) -> *const c_char;
    pub fn sqlite3_errmsg16(db: *mut SQLite3) -> *const c_void;
    pub fn sqlite3_errstr(code: c_int) -> *const c_char;
    pub fn sqlite3_error_offset(db: *mut SQLite3) -> c_int;
}
