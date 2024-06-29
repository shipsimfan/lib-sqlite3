use crate::SQLite3;
use std::ffi::{c_char, c_int, c_void};

#[link(name = "sqlite3")]
extern "C" {
    pub fn sqlite3_exec(
        db: *mut SQLite3,
        sql: *const c_char,
        callback: Option<extern "C" fn(*mut c_void, c_int, *mut *mut c_char, *mut *mut c_char)>,
        callback_arg: *mut c_void,
        errmsg: *mut *mut c_char,
    ) -> c_int;
}
