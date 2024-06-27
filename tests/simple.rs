use std::{ffi::CStr, ptr::null_mut};

use sqlite3::sqlite3_open;

#[test]
fn simple_open() {
    const FILENAME: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"test.db\0") };

    let mut db = null_mut();

    let result = unsafe { sqlite3_open(FILENAME.as_ptr(), &mut db) };

    assert_eq!(result, 0);
}
