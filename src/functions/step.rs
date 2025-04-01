use crate::SQLite3Stmt;
use std::ffi::c_int;

#[cfg_attr(target_os = "windows", link(name = "sqlite3-win"))]

extern "C" {
    pub fn sqlite3_step(stmt: *mut SQLite3Stmt) -> c_int;
}
