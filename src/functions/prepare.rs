use crate::{SQLite3, SQLite3Stmt};
use std::ffi::{c_char, c_int, c_uint, c_void};

#[link(name = "sqlite3")]
extern "C" {
    pub fn sqlite3_prepare(
        db: *mut SQLite3,
        sql: *const c_char,
        byte: c_int,
        stmt: *mut *mut SQLite3Stmt,
        tail: *mut *const c_char,
    ) -> c_int;
    pub fn sqlite3_prepare_v2(
        db: *mut SQLite3,
        sql: *const c_char,
        byte: c_int,
        stmt: *mut *mut SQLite3Stmt,
        tail: *mut *const c_char,
    ) -> c_int;
    pub fn sqlite3_prepare_v3(
        db: *mut SQLite3,
        sql: *const c_char,
        byte: c_int,
        prep_flags: c_uint,
        stmt: *mut *mut SQLite3Stmt,
        tail: *mut *const c_char,
    ) -> c_int;
    pub fn sqlite3_prepare16(
        db: *mut SQLite3,
        sql: *const c_void,
        byte: c_int,
        stmt: *mut *mut SQLite3Stmt,
        tail: *mut *const c_void,
    ) -> c_int;
    pub fn sqlite3_prepare16_v2(
        db: *mut SQLite3,
        sql: *const c_void,
        byte: c_int,
        stmt: *mut *mut SQLite3Stmt,
        tail: *mut *const c_void,
    ) -> c_int;
    pub fn sqlite3_prepare16_v3(
        db: *mut SQLite3,
        sql: *const c_void,
        byte: c_int,
        prep_flags: c_uint,
        stmt: *mut *mut SQLite3Stmt,
        tail: *mut *const c_void,
    ) -> c_int;
}
