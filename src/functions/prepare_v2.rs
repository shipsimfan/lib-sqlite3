use crate::{SQLite3, SQLite3Stmt};
use std::ffi::{c_char, c_int};

#[link(name = "sqlite3")]
extern "C" {
    #[allow(missing_docs)]
    pub fn sqlite3_prepare_v2(
        db: *mut SQLite3,
        sql: *const c_char,
        byte: c_int,
        stmt: *mut *mut SQLite3Stmt,
        tail: *mut *const c_char,
    ) -> c_int;
}
