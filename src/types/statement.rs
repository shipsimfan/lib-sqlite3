use std::ffi::c_void;

/// An instance of this object represents a single SQL statement that has been compiled into binary
/// form and is ready to be evaluated.
///
/// Think of each SQL statement as a separate computer program. The original SQL text is source
/// code. A prepared statement object is the compiled object code. All SQL must be converted into a
/// prepared statement before it can be run.
///
/// The life-cycle of a prepared statement object usually goes like this:
///  - Create the prepared statement object using [`sqlite3_prepare_v2`].
///  - Bind values to parameters using the `sqlite3_bind_*` interfaces.
///  - Run the SQL by calling [`sqlite3_step`] one or more times.
///  - Reset the prepared statement using [`sqlite3_reset`] then go back to step 2. Do this zero or more times.
///  - Destroy the object using [`sqlite3_finalize`].
pub type SQLite3Stmt = c_void;
