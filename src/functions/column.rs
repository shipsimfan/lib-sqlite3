use crate::{SQLite3Int64, SQLite3Stmt};
use std::ffi::{c_double, c_int, c_uchar, c_void};

#[link(name = "sqlite3")]
extern "C" {
    pub fn sqlite3_column_blob(stmt: *mut SQLite3Stmt, col: c_int) -> *const c_void;
    pub fn sqlite3_column_double(stmt: *mut SQLite3Stmt, col: c_int) -> c_double;
    pub fn sqlite3_column_int(stmt: *mut SQLite3Stmt, col: c_int) -> c_int;
    pub fn sqlite3_column_int64(stmt: *mut SQLite3Stmt, col: c_int) -> SQLite3Int64;
    pub fn sqlite3_column_text(stmt: *mut SQLite3Stmt, col: c_int) -> *const c_uchar;
    pub fn sqlite3_column_text16(stmt: *mut SQLite3Stmt, col: c_int) -> *const c_void;
    pub fn sqlite3_column_bytes(stmt: *mut SQLite3Stmt, col: c_int) -> c_int;
    pub fn sqlite3_column_bytes16(stmt: *mut SQLite3Stmt, col: c_int) -> c_int;
    pub fn sqlite3_column_type(stmt: *mut SQLite3Stmt, col: c_int) -> c_int;
}
