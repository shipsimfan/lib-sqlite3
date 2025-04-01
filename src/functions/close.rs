use crate::SQLite3;
use std::ffi::c_int;

#[cfg_attr(target_os = "windows", link(name = "sqlite3-win"))]

extern "C" {
    pub fn sqlite3_close(db: *mut SQLite3) -> c_int;
    pub fn sqlite3_close_v2(db: *mut SQLite3) -> c_int;
}
