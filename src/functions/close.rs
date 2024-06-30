use crate::SQLite3;
use std::ffi::c_int;

#[link(name = "sqlite3")]
extern "C" {
    pub fn sqlite3_close(db: *mut SQLite3) -> c_int;
    pub fn sqlite3_close_v2(db: *mut SQLite3) -> c_int;
}
