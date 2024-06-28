use crate::SQLite3;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SQLITE_BUSY, SQLITE_OK};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "sqlite3")]
extern "C" {
    /// The [`sqlite3_close`] routine is a destructor for the [`SQLite3`] object. Calls to
    /// [`sqlite3_close`] return [`SQLITE_OK`] if the [`SQLite3`] object is successfully destroyed
    /// and all associated resources are deallocated.
    ///
    /// Ideally, applications should finalize all prepared statements, close all [`BLOB`] handles,
    /// and finish all [`sqlite3_backup`] objects associated with the [`SQLite3`] object prior to
    /// attempting to close the object. If the database connection is associated with unfinalized
    /// prepared statements, [`BLOB`] handlers, and/or unfinished [`sqlite3_backup`] objects then
    /// [`sqlite3_close`] will leave the database connection open and return [`SQLITE_BUSY`].
    ///
    /// If an [`SQLite3`] object is destroyed while a transaction is open, the transaction is
    /// automatically rolled back.
    ///
    /// The `db` parameter to [`sqlite3_close`] must be either a [`null_mut`] pointer or an
    /// [`SQLite3`] object pointer obtained from [`sqlite3_open`], [`sqlite3_open16`], or
    /// [`sqlite3_open_v2`], and not previously closed. Calling [`sqlite3_close`] with a
    /// [`null_mut`] pointer argument is a harmless no-op.
    pub fn sqlite3_close(db: *mut SQLite3) -> c_int;
}
