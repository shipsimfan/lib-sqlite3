use crate::SQLite3Stmt;
use std::ffi::c_int;

#[link(name = "sqlite3")]
extern "C" {
    pub fn sqlite3_finalize(stmt: *mut SQLite3Stmt) -> c_int;
}
