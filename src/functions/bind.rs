use crate::{SQLite3Int64, SQLite3Stmt, SQLite3UInt64};
use std::ffi::{c_char, c_double, c_int, c_void};

#[link(name = "sqlite3")]
extern "C" {
    pub fn sqlite3_bind_blob(
        stmt: *mut SQLite3Stmt,
        i: c_int,
        blob: *const c_void,
        n: c_int,
        destructor: extern "C" fn(*mut c_void),
    ) -> c_int;
    pub fn sqlite3_bind_blob64(
        stmt: *mut SQLite3Stmt,
        i: c_int,
        blob: *const c_void,
        n: SQLite3UInt64,
        destructor: extern "C" fn(*mut c_void),
    ) -> c_int;
    pub fn sqlite3_bind_double(stmt: *mut SQLite3Stmt, i: c_int, val: c_double) -> c_int;
    pub fn sqlite3_bind_int(stmt: *mut SQLite3Stmt, i: c_int, val: c_int) -> c_int;
    pub fn sqlite3_bind_int64(stmt: *mut SQLite3Stmt, i: c_int, val: SQLite3Int64) -> c_int;
    pub fn sqlite3_bind_null(stmt: *mut SQLite3Stmt, i: c_int) -> c_int;
    pub fn sqlite3_bind_text(
        stmt: *mut SQLite3Stmt,
        i: c_int,
        text: *const c_char,
        n: c_int,
        destructor: extern "C" fn(*mut c_void),
    ) -> c_int;
    pub fn sqlite3_bind_text16(
        stmt: *mut SQLite3Stmt,
        i: c_int,
        text: *const c_void,
        n: c_int,
        destructor: extern "C" fn(*mut c_void),
    ) -> c_int;
    pub fn sqlite3_bind_text64(
        stmt: *mut SQLite3Stmt,
        i: c_int,
        text: *const c_char,
        n: SQLite3Int64,
        destructor: extern "C" fn(*mut c_void),
    ) -> c_int;
    pub fn sqlite3_bind_pointer(
        stmt: *mut SQLite3Stmt,
        i: c_int,
        ptr: *mut c_void,
        r#type: *const c_char,
        destructor: extern "C" fn(*mut c_void),
    ) -> c_int;
    pub fn sqlite3_bind_zeroblob(stmt: *mut SQLite3Stmt, i: c_int, n: c_int) -> c_int;
    pub fn sqlite3_bind_zeroblob64(stmt: *mut SQLite3Stmt, i: c_int, n: SQLite3UInt64) -> c_int;

}
