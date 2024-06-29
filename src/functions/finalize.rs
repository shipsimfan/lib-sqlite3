use crate::SQLite3Stmt;
use std::ffi::c_int;

#[link(name = "sqlite3")]
extern "C" {
    #[allow(missing_docs)]
    pub fn sqlite3_finalize(stmt: *mut SQLite3Stmt) -> c_int;
}
