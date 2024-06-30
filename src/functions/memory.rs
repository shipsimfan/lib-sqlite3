use crate::SQLite3UInt64;
use std::ffi::{c_int, c_void};

#[link(name = "sqlite3")]
extern "system" {
    pub fn sqlite3_malloc(n: c_int) -> *mut c_void;
    pub fn sqlite3_malloc64(n: SQLite3UInt64) -> *mut c_void;
    pub fn sqlite3_realloc(ptr: *mut c_void, n: c_int) -> *mut c_void;
    pub fn sqlite3_realloc64(ptr: *mut c_void, b: SQLite3UInt64) -> *mut c_void;
    pub fn sqlite3_free(ptr: *mut c_void);
    pub fn sqlite3_msize(ptr: *mut c_void) -> SQLite3UInt64;
}
