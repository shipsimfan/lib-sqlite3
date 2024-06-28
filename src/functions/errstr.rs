use std::ffi::{c_char, c_int};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "sqlite3")]
extern "C" {
    /// The [`sqlite3_errstr`] interface returns the English-language text that describes the
    /// result code `code`, as UTF-8, or [`null`] if `code` is not an result code for which a text
    /// error message is available. Memory to hold the error message string is managed internally
    /// and must not be freed by the application.
    pub fn sqlite3_errstr(code: c_int) -> *const c_char;
}
